use path_clean::PathClean;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::path::{Path, PathBuf};

pub trait IoUtil {
    /// Create a file if a file with the same name doesn't exist.
    /// If some of the intermediate directories are missing, they will be created.
    #[allow(unused)]
    fn create_file_if_nonexistent(&mut self, filepath: &Path) -> Result<Box<dyn Write>> {
        unreachable!()
    }
    /// Open a file for reading.
    #[allow(unused)]
    fn open_file_for_read(&self, filepath: &Path) -> Result<Box<dyn Read>> {
        unreachable!()
    }
    /// Make a directory at path.
    /// If some of the intermediate directories are missing, they will be created.
    #[allow(unused)]
    fn mkdir_p(&mut self, path: &Path) -> Result<()> {
        unreachable!()
    }
    /// mock for write!(file, "{}", str)
    #[allow(unused)]
    fn write_str_to_file(&self, file: &mut dyn Write, s: &str) -> Result<()> {
        unreachable!()
    }
    /// mock for read_to_end
    #[allow(unused)]
    fn read_from_file(&self, file: &mut dyn Read) -> Result<String> {
        unreachable!()
    }
    /// Get the absolute path.
    #[allow(unused)]
    fn to_absolute(&self, path: &Path) -> Result<PathBuf> {
        unreachable!()
    }
    /// List a directory. Return a list of paths relative to `path`.
    #[allow(unused)]
    fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>> {
        unreachable!()
    }
    /// Remove directory after recursively deleting its contents.
    #[allow(unused)]
    fn remove_dir_all(&self, path: &Path) -> Result<()> {
        unreachable!()
    }
}

pub trait IoUtilExt {}
impl<T: IoUtilExt> IoUtil for T {
    /// Create a file if a file with the same name doesn't exist.
    /// If some of the intermediate directories are missing, they will be created.
    fn create_file_if_nonexistent(&mut self, filepath: &Path) -> Result<Box<dyn Write>> {
        let filepath = self.to_absolute(filepath)?;
        if let Some(parent) = filepath.parent() {
            self.mkdir_p(&parent)?;
        }
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(filepath)?;
        Ok(Box::new(file))
    }
    fn open_file_for_read(&self, filepath: &Path) -> Result<Box<dyn Read>> {
        let file = OpenOptions::new().read(true).open(filepath)?;
        Ok(Box::new(file))
    }
    fn mkdir_p(&mut self, path: &Path) -> Result<()> {
        let path = self.to_absolute(path)?;
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
    fn read_from_file(&self, file: &mut dyn Read) -> Result<String> {
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        // TODO: handle encoding errors correctly
        let s = String::from_utf8(buf).unwrap();
        Ok(s)
    }
    fn to_absolute(&self, path: &Path) -> Result<PathBuf> {
        let path = if path.is_absolute() {
            path.to_owned()
        } else {
            std::env::current_dir()?.join(path)
        }
        .clean();
        Ok(path)
    }
    fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let mut result = vec![];
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            // We are interested in files only.
            if entry.file_type()?.is_file() {
                // We need to return relative paths.
                result.push(entry.file_name().into());
            }
        }
        Ok(result)
    }
    fn remove_dir_all(&self, path: &Path) -> Result<()> {
        std::fs::remove_dir_all(path)
    }
}

pub struct IoUtilImpl;

impl IoUtilExt for IoUtilImpl {}
