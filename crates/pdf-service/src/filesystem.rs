//! Filesystem helpers shared by every tool.
//!
//! Safety rules from the spec: never overwrite the user's original, and never
//! silently overwrite an existing output. These helpers generate new names
//! instead.

use std::path::{Path, PathBuf};

/// Build a path in `directory` for `stem.extension` that does not exist yet.
///
/// Collisions get a numeric suffix: `report.pdf`, `report_1.pdf`, ...
pub fn unique_path(directory: &Path, stem: &str, extension: &str) -> PathBuf {
    let extension = extension.trim_start_matches('.');
    let first = directory.join(format!("{stem}.{extension}"));
    if !first.exists() {
        return first;
    }
    for index in 1..10_000u32 {
        let candidate = directory.join(format!("{stem}_{index}.{extension}"));
        if !candidate.exists() {
            return candidate;
        }
    }
    directory.join(format!("{stem}_{}.{}", std::process::id(), extension))
}

/// Pick a stem for a multi-file output (e.g. Split) whose `_001` file is free.
pub fn unique_run_stem(directory: &Path, base: &str, extension: &str) -> String {
    let extension = extension.trim_start_matches('.');
    if run_is_free(directory, base, extension) {
        return base.to_string();
    }
    for index in 1..10_000u32 {
        let candidate = format!("{base}_{index}");
        if run_is_free(directory, &candidate, extension) {
            return candidate;
        }
    }
    format!("{base}_{}", std::process::id())
}

fn run_is_free(directory: &Path, stem: &str, extension: &str) -> bool {
    !directory.join(format!("{stem}_001.{extension}")).exists()
}
