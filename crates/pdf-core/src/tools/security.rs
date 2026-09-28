//! Security tools: Protect PDF (encrypt with a password) and Unlock PDF
//! (remove protection using a known password).
//!
//! Unlock never attempts to guess, brute-force, or bypass a password; it only
//! authenticates the exact password the user provides.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

use lopdf::encryption::crypt_filters::{Aes128CryptFilter, CryptFilter};
use lopdf::{Document, EncryptionState, EncryptionVersion, LoadOptions, Permissions};

use crate::error::{PdfCoreError, PdfCoreResult};

/// Options for [`protect`].
pub struct ProtectOptions {
    /// Password required to open the document.
    pub user_password: String,
    /// Password that grants full permissions. Defaults to the user password.
    pub owner_password: Option<String>,
    pub allow_printing: bool,
    pub allow_copying: bool,
    pub allow_modifying: bool,
}

impl ProtectOptions {
    fn permissions(&self) -> Permissions {
        let mut permissions = Permissions::empty();
        if self.allow_printing {
            permissions |= Permissions::PRINTABLE | Permissions::PRINTABLE_IN_HIGH_QUALITY;
        }
        if self.allow_copying {
            permissions |= Permissions::COPYABLE | Permissions::COPYABLE_FOR_ACCESSIBILITY;
        }
        if self.allow_modifying {
            permissions |= Permissions::MODIFIABLE
                | Permissions::ANNOTABLE
                | Permissions::FILLABLE
                | Permissions::ASSEMBLABLE;
        }
        permissions
    }
}

/// Encrypt `input` with a password (AES-128, the standard PDF security handler)
/// and write it to `output`. Returns the page count.
pub fn protect(input: &Path, options: &ProtectOptions, output: &Path) -> PdfCoreResult<u32> {
    if options.user_password.is_empty() {
        return Err(PdfCoreError::Other(
            "enter a password to protect the document".to_string(),
        ));
    }

    let mut document = Document::load_with_options(
        input,
        LoadOptions::with_max_decompressed_size(crate::loading::MAX_DECOMPRESSED_BYTES),
    )
    .map_err(map_load(input))?;
    if document.is_encrypted() {
        return Err(PdfCoreError::Other(
            "this PDF is already protected; unlock it first".to_string(),
        ));
    }
    let total = document.get_pages().len() as u32;
    ensure_file_id(&mut document);

    let owner_password = options
        .owner_password
        .clone()
        .unwrap_or_else(|| options.user_password.clone());

    let crypt_filter: Arc<dyn CryptFilter> = Arc::new(Aes128CryptFilter);
    let version = EncryptionVersion::V4 {
        document: &document,
        encrypt_metadata: true,
        crypt_filters: BTreeMap::from([(b"StdCF".to_vec(), crypt_filter)]),
        stream_filter: b"StdCF".to_vec(),
        string_filter: b"StdCF".to_vec(),
        owner_password: &owner_password,
        user_password: &options.user_password,
        permissions: options.permissions(),
    };

    let state = EncryptionState::try_from(version)
        .map_err(|error| PdfCoreError::Other(format!("could not encrypt the document: {error}")))?;
    document
        .encrypt(&state)
        .map_err(|error| PdfCoreError::Other(format!("could not encrypt the document: {error}")))?;
    document
        .save(output)
        .map_err(|error| PdfCoreError::Other(format!("failed to write PDF: {error}")))?;

    Ok(total)
}

/// Remove protection using the correct password and write a plain PDF.
///
/// Returns [`PdfCoreError::WrongPassword`] when the password is incorrect.
pub fn unlock(input: &Path, password: &str, output: &Path) -> PdfCoreResult<u32> {
    if password.is_empty() {
        return Err(PdfCoreError::Other(
            "enter the document password".to_string(),
        ));
    }

    let mut document = Document::load_with_options(
        input,
        LoadOptions {
            password: Some(password.to_string()),
            max_decompressed_size: Some(crate::loading::MAX_DECOMPRESSED_BYTES),
            ..Default::default()
        },
    )
    .map_err(|error| map_unlock_error(input, error))?;

    if !document.was_encrypted() {
        return Err(PdfCoreError::Other(
            "this PDF is not password-protected".to_string(),
        ));
    }

    let total = document.get_pages().len() as u32;
    document
        .save(output)
        .map_err(|error| PdfCoreError::Other(format!("failed to write PDF: {error}")))?;
    Ok(total)
}

fn map_load(path: &Path) -> impl Fn(lopdf::Error) -> PdfCoreError + '_ {
    move |error| match &error {
        lopdf::Error::IO(io) => PdfCoreError::Io {
            path: path.to_path_buf(),
            source: std::io::Error::new(io.kind(), io.to_string()),
        },
        _ => PdfCoreError::from_engine(error),
    }
}

/// Encrypting requires a file `/ID`; add a stable random one when a malformed
/// or minimal document is missing it.
fn ensure_file_id(document: &mut Document) {
    if document.trailer.get(b"ID").is_ok() {
        return;
    }

    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    let seed = nanos ^ (u128::from(std::process::id()) << 32);
    let mut bytes = [0u8; 16];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = ((seed >> ((index % 16) * 8)) & 0xff) as u8 ^ (index as u8).wrapping_mul(31);
    }

    let id = lopdf::Object::string_literal(bytes.to_vec());
    document.trailer.set("ID", vec![id.clone(), id]);
}

fn map_unlock_error(path: &Path, error: lopdf::Error) -> PdfCoreError {
    if let lopdf::Error::IO(io) = &error {
        return PdfCoreError::Io {
            path: path.to_path_buf(),
            source: std::io::Error::new(io.kind(), io.to_string()),
        };
    }
    let message = error.to_string().to_ascii_lowercase();
    if message.contains("password") || message.contains("decrypt") {
        PdfCoreError::WrongPassword
    } else if message.contains("encrypt") {
        PdfCoreError::PasswordRequired
    } else {
        PdfCoreError::InvalidPdf(error.to_string())
    }
}
