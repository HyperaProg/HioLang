/*
 * HioClib Library Example - File I/O in Rust
 * Safe file operations for Hiolang
 */

use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

/// Read entire file as string
pub fn hio_read_file(path: &str) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Write string to file
pub fn hio_write_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Check if file exists
pub fn hio_file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// Get file size in bytes
pub fn hio_file_size(path: &str) -> io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

/// Append to file
pub fn hio_append_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Delete file
pub fn hio_delete_file(path: &str) -> io::Result<()> {
    fs::remove_file(path)?;
    Ok(())
}

/// List files in directory
pub fn hio_list_directory(path: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        if let Some(name) = file_name.to_str() {
            files.push(name.to_string());
        }
    }
    Ok(files)
}
