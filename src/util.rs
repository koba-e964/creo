use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

/// Create a file if a file with the same name doesn't exist.
/// If some of the intermediate directories are missing, they are created.
pub fn create_file_if_nonexistent(filepath: &Path) -> Result<File> {
    let filepath = if filepath.is_absolute() {
        filepath.to_owned()
    } else {
        filepath.canonicalize()?
    };
    if let Some(parent) = filepath.parent() {
        mkdir_p(&parent)?;
    }
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filepath)
}

fn mkdir_p(path: &Path) -> Result<()> {
    if path.is_dir() {
        return Ok(());
    }
    if !path.exists() {
        if let Some(parent) = path.parent() {
            mkdir_p(&parent)?;
        }
        std::fs::create_dir(path)?;
        return Ok(());
    }
    Err(Error::new(ErrorKind::Other, "not a directory"))
}
