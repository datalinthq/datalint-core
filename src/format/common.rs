use std::path::PathBuf;

/// Check if a file is a JSON file based on its extension.
///
/// # Arguments
/// * `file` - A reference to a `PathBuf` representing the file to check.
///
/// # Returns
/// * `true` if the file has a `.json` extension, `false` otherwise.
pub fn is_json_file(file: &PathBuf) -> bool {
    file.extension().map_or(false, |ext| ext == "json")
}
