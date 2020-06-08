use std::io::{Error, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::io_util::IoUtil;

/// Utility trait for compiling/running executables.
pub trait RunUtil {
    /// Compiles a file into a temporary file and returns the path to the temporary file.
    #[allow(unused)]
    fn compile(&mut self, cd: &Path, src: &Path, compile: &[String]) -> Result<PathBuf> {
        unreachable!()
    }
    /// Runs an executable once.
    #[allow(unused)]
    fn run_once(&mut self, cd: &Path, exec: &Path) -> Result<()> {
        unreachable!()
    }
    /// Runs an executable with an input file.
    #[allow(unused)]
    fn run_with_input(&mut self, cd: &Path, exec: &Path, infile: &Path) -> Result<()> {
        unreachable!()
    }
    /// Runs an executable with an input file and write its output to a file.
    #[allow(unused)]
    fn run_pipe(&mut self, cd: &Path, exec: &Path, infile: &Path, outfile: &Path) -> Result<()> {
        unreachable!()
    }
}

pub struct RunUtilImpl {
    pub io_util: Box<dyn IoUtil>,
}

impl RunUtil for RunUtilImpl {
    fn compile(&mut self, cd: &Path, src: &Path, compile: &[String]) -> Result<PathBuf> {
        let outpath = self.io_util.to_absolute(&PathBuf::from("rand_TODO"))?;
        let prog = &compile[0];
        let mut args = compile[1..].to_vec();
        for v in args.iter_mut() {
            if *v == "$IN" {
                *v = src.to_str().unwrap().to_owned();
            }
            if *v == "$OUT" {
                *v = outpath.to_str().unwrap().to_owned();
            }
        }
        let status = Command::new(prog).current_dir(cd).args(&args).status()?;
        if !status.success() {
            eprintln!("compile status = {}", status);
            if let Some(exit_code) = status.code() {
                return Err(Error::from_raw_os_error(exit_code));
            } else {
                return Err(Error::from_raw_os_error(128));
            }
        }
        Ok(outpath)
    }
    fn run_once(&mut self, cd: &Path, exec: &Path) -> Result<()> {
        let status = Command::new(exec).current_dir(cd).status()?;
        if !status.success() {
            if let Some(exit_code) = status.code() {
                return Err(Error::from_raw_os_error(exit_code));
            } else {
                return Err(Error::from_raw_os_error(128));
            }
        }
        Ok(())
    }
    fn run_with_input(&mut self, _cd: &Path, _exec: &Path, _infile: &Path) -> Result<()> {
        todo!()
    }
    fn run_pipe(
        &mut self,
        _cd: &Path,
        _exec: &Path,
        _infile: &Path,
        _outfile: &Path,
    ) -> Result<()> {
        todo!()
    }
}
