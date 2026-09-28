//! Text shaping and font embedding for generated PDF content.
//!
//! Arabic must render with correct contextual forms, right-to-left ordering,
//! and an embedded font — never disconnected letters. We use `rustybuzz`
//! (HarfBuzz port, OpenType shaping) and `ttf-parser`, and embed a full
//! TrueType font as a Type0/CIDFontType2 with `Identity-H` encoding, so the
//! produced PDF is self-contained and renders identically everywhere.
//!
//! The bundled font is **Amiri** (SIL Open Font License 1.1); see
//! `assets/fonts/OFL.txt` and `THIRD_PARTY_LICENSES.md`.

use lopdf::{dictionary, Document, Object, ObjectId, Stream};
use ttf_parser::Face as TtfFace;

use crate::error::{PdfCoreError, PdfCoreResult};

/// Bundled Arabic-capable font (Amiri, OFL-1.1).
pub const EMBEDDED_FONT: &[u8] = include_bytes!("../../../assets/fonts/Amiri-Regular.ttf");

/// The PostScript name used for the embedded font in the PDF.
pub const EMBEDDED_FONT_NAME: &str = "Amiri-Regular";

/// Text shaped into the glyph ids and advance width needed to write it.
pub struct ShapedText {
    /// Glyph ids in visual order, ready to be written as an `Identity-H` string.
    pub glyphs: Vec<u16>,
    /// Total advance width at the requested size, in points.
    pub advance: f32,
}

impl ShapedText {
    /// The glyph ids as a PDF hex string: `<00010002...>`.
    pub fn hex(&self) -> String {
        let mut out = String::with_capacity(self.glyphs.len() * 4 + 2);
        out.push('<');
        for glyph in &self.glyphs {
            out.push_str(&format!("{glyph:04X}"));
        }
        out.push('>');
        out
    }
}

/// Shape `text` with the given font at the given size.
///
/// Direction, script, and language are guessed from the text, so Arabic is
/// shaped and ordered correctly while Latin stays left-to-right.
pub fn shape_text(font_bytes: &[u8], text: &str, size: f32) -> PdfCoreResult<ShapedText> {
    let face = rustybuzz::Face::from_slice(font_bytes, 0)
        .ok_or_else(|| PdfCoreError::Other("could not read the embedded font".to_string()))?;

    let mut buffer = rustybuzz::UnicodeBuffer::new();
    buffer.push_str(text);
    buffer.guess_segment_properties();

    let output = rustybuzz::shape(&face, &[], buffer);
    let glyphs: Vec<u16> = output
        .glyph_infos()
        .iter()
        .map(|info| u16::try_from(info.glyph_id).unwrap_or(0))
        .collect();

    let units_per_em = TtfFace::parse(font_bytes, 0)
        .map(|parsed| parsed.units_per_em() as f32)
        .unwrap_or(1000.0);
    let advance_units: f32 = output
        .glyph_positions()
        .iter()
        .map(|position| position.x_advance as f32)
        .sum();
    let advance = if units_per_em > 0.0 {
        advance_units / units_per_em * size
    } else {
        0.0
    };

    Ok(ShapedText { glyphs, advance })
}

/// Embed the font into `document` and return the object id of the Type0 font.
pub fn embed_font(
    document: &mut Document,
    font_bytes: &[u8],
    base_name: &str,
) -> PdfCoreResult<ObjectId> {
    let face = TtfFace::parse(font_bytes, 0)
        .map_err(|error| PdfCoreError::Other(format!("could not parse the font: {error}")))?;
    let units_per_em = face.units_per_em().max(1) as f32;
    let scale = |value: i16| (value as f32) * 1000.0 / units_per_em;

    // Widths for every glyph, so the W array always covers the used glyphs.
    let glyph_count = face.number_of_glyphs();
    let mut widths: Vec<Object> = Vec::with_capacity(glyph_count as usize);
    for index in 0..glyph_count {
        let advance = face
            .glyph_hor_advance(ttf_parser::GlyphId(index))
            .unwrap_or(0);
        let width = (advance as f32) * 1000.0 / units_per_em;
        widths.push(Object::Integer(width.round() as i64));
    }

    let bounding_box = face.global_bounding_box();
    let font_bbox = vec![
        Object::Integer(scale(bounding_box.x_min).round() as i64),
        Object::Integer(scale(bounding_box.y_min).round() as i64),
        Object::Integer(scale(bounding_box.x_max).round() as i64),
        Object::Integer(scale(bounding_box.y_max).round() as i64),
    ];
    let ascent = scale(face.ascender()).round() as i64;
    let descent = scale(face.descender()).round() as i64;

    let font_file_id = document.add_object(Stream::new(
        dictionary! {
            "Length1" => font_bytes.len() as i64,
        },
        font_bytes.to_vec(),
    ));

    let descriptor_id = document.add_object(dictionary! {
        "Type" => "FontDescriptor",
        "FontName" => base_name,
        "Flags" => 4,
        "FontBBox" => font_bbox,
        "ItalicAngle" => 0,
        "Ascent" => ascent,
        "Descent" => descent,
        "CapHeight" => ascent,
        "StemV" => 80,
        "FontFile2" => font_file_id,
    });

    let cid_font_id = document.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "CIDFontType2",
        "BaseFont" => base_name,
        "CIDSystemInfo" => dictionary! {
            "Registry" => Object::string_literal("Adobe"),
            "Ordering" => Object::string_literal("Identity"),
            "Supplement" => 0,
        },
        "FontDescriptor" => descriptor_id,
        "CIDToGIDMap" => "Identity",
        "DW" => 1000,
        "W" => vec![Object::Integer(0), Object::Array(widths)],
    });

    let type0_id = document.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type0",
        "BaseFont" => base_name,
        "Encoding" => "Identity-H",
        "DescendantFonts" => vec![Object::Reference(cid_font_id)],
    });

    Ok(type0_id)
}
