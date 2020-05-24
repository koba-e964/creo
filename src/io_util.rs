use path_clean::PathClean;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Result, Write};
use std::path::{Path, PathBuf};

pub trait IoUtil {
    /// Create a file if a file with the same name doesn't exist.
    /// If some of the intermediate directories are missing, they will be created.
    fn create_file_if_nonexistent(&mut self, filepath: &Path) -> Result<Box<dyn Write>>;
    /// Make a directory at path.
    /// If some of the intermediate directories are missing, they will be created.
    fn mkdir_p(&mut self, path: &Path) -> Result<()>;
    /// mock for write!(file, "{}", str)
    fn write_str_to_file(&self, file: &mut dyn Write, s: &str) -> Result<()>;
}

pub struct IoUtilImpl;

impl IoUtil for IoUtilImpl {
    /// Create a file if a file with the same name doesn't exist.
    /// If some of the intermediate directories are missing, they will be created.
    fn create_file_if_nonexistent(&mut self, filepath: &Path) -> Result<Box<dyn Write>> {
        let filepath = to_absolute(filepath)?;
        if let Some(parent) = filepath.parent() {
            self.mkdir_p(&parent)?;
        }
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(filepath)?;
        Ok(Box::new(file))
    }
    fn mkdir_p(&mut self, path: &Path) -> Result<()> {
        let path = to_absolute(path)?;
        if path.is_dir() {
            return Ok(());
        }
        if !path.exists() {
            if let Some(parent) = path.parent() {
                self.mkdir_p(&parent)?;
            }
            std::fs::create_dir(path)?;
            return Ok(());
        }
        Err(Error::new(
            ErrorKind::Other,
            format!("not a directory: {}", path.display()),
        ))
    }
    fn write_str_to_file(&self, file: &mut dyn Write, s: &str) -> Result<()> {
        write!(file, "{}", s)
    }
}

fn to_absolute(path: &Path) -> Result<PathBuf> {
    let path = if path.is_absolute() {
        path.to_owned()
    } else {
        std::env::current_dir()?.join(path)
    }
    .clean();
    Ok(path)
}
