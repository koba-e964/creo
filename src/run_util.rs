use sha2::{Digest, Sha256};
use std::io::{Error as IOError, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::error::Result;
use crate::io_util::{IoUtil, IoUtilExt};

/// Utility trait for compiling/running executables.
pub trait RunUtil {
    /// Compiles a file into a temporary file and returns the path to the temporary file.
    #[allow(unused)]
    fn compile(&mut self, cd: &Path, src: &Path, compile: &[String]) -> Result<PathBuf> {
        unreachable!()
    }
    /// Runs an executable once.
    #[allow(unused)]
    fn run_once(&mut self, cd: &Path, exec: &Path, run: &[String]) -> Result<()> {
        unreachable!()
    }
    /// Runs an executable with an input file, returning its output.
    #[allow(unused)]
    fn run_with_input(
        &mut self,
        cd: &Path,
        exec: &Path,
        run: &[String],
        infile: &Path,
    ) -> Result<Vec<u8>> {
        unreachable!()
    }
    /// Runs an executable with an input file and write its output to a file.
    #[allow(unused)]
    fn run_pipe(
        &mut self,
        cd: &Path,
        exec: &Path,
        run: &[String],
        infile: &Path,
        outfile: &Path,
    ) -> Result<()> {
        unreachable!()
    }
}

pub trait RunUtilExt: IoUtil {}
impl<T: RunUtilExt> RunUtil for T {
    fn compile(&mut self, cd: &Path, src: &Path, compile: &[String]) -> Result<PathBuf> {
        let tempdir = Path::new("/tmp/creo-cache/");
        self.mkdir_p(tempdir)?;
        // Compute a hash value from compile and the content of src.
        let mut hash_str = String::with_capacity(64);
        {
            let mut handle = self.open_file_for_read(src)?;
            let content = self.read_bytes_from_file(&mut handle)?;
            let mut hasher: Sha256 = Sha256::new();
            for c in compile {
                hasher.input(c.as_bytes());
            }
            hasher.input(&content);
            let hash_val = hasher.result();
            for &val in &hash_val {
                hash_str += &format!("{:02x}", val);
            }
        }
        let outpath = tempdir.join(hash_str);
        // If there exists an already compiled binary, return early.
        if outpath.is_file() {
            eprintln!(
                "File {} exists: skipping compilation (source: {})",
                outpath.to_str().unwrap(),
                src.display(),
            )
        }
        let mut compile = compile.to_vec();
        for v in compile.iter_mut() {
            if *v == "$IN" {
                *v = src.to_str().unwrap().to_owned();
            }
            if *v == "$OUT" {
                *v = outpath.to_str().unwrap().to_owned();
            }
        }
        let prog = &compile[0];
        let args = compile[1..].to_vec();
        let status = Command::new(prog).current_dir(cd).args(&args).status()?;
        if !status.success() {
            eprintln!("compile status = {}", status);
            let err = IOError::new(
                ErrorKind::InvalidData,
                format!(
                    "Compiling {} failed (cd = {}, options = {:?})",
                    src.display(),
                    cd.display(),
                    compile
                ),
            );
            return Err(err.into());
        }
        Ok(outpath)
    }
    fn run_once(&mut self, cd: &Path, exec: &Path, run: &[String]) -> Result<()> {
        let mut run = run.to_vec();
        for v in run.iter_mut() {
            if *v == "$OUT" {
                *v = exec.to_str().unwrap().to_owned();
            }
        }
        let prog = &run[0];
        let args = run[1..].to_vec();
        let status = Command::new(prog).args(&args).current_dir(cd).status()?;
        if !status.success() {
            let err = IOError::new(
                ErrorKind::InvalidData,
                format!(
                    "Executing {} failed (cd = {}, options = {:?})",
                    exec.display(),
                    cd.display(),
                    run
                ),
            );
            return Err(err.into());
        }
        Ok(())
    }
    fn run_with_input(
        &mut self,
        cd: &Path,
        exec: &Path,
        run: &[String],
        infile: &Path,
    ) -> Result<Vec<u8>> {
        let mut run = run.to_vec();
        for v in run.iter_mut() {
            if *v == "$OUT" {
                *v = exec.to_str().unwrap().to_owned();
            }
        }
        let prog = &run[0];
        let args = run[1..].to_vec();
        let mut child = Command::new(prog)
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .current_dir(cd)
            .spawn()?;
        let inproc;
        {
            let mut file = self.open_file_for_read(infile)?;
            let content = self.read_bytes_from_file(&mut file)?;
            // Give stdin in a separate thread
            let mut stdin = child.stdin.take().expect("Failed to get stdin");
            inproc = std::thread::spawn(move || {
                stdin.write_all(&content).expect("Failed to write content");
            });
        }
        let output = child.wait_with_output()?;
        inproc.join().unwrap();
        if !output.status.success() {
            eprintln!("run status = {}", output.status);
            let err = IOError::new(
                ErrorKind::InvalidData,
                format!(
                    "Running {} failed (cd = {}, options = {:?})",
                    exec.display(),
                    cd.display(),
                    run,
                ),
            );
            return Err(err.into());
        }

        let stdout = output.stdout;
        Ok(stdout)
    }
    fn run_pipe(
        &mut self,
        cd: &Path,
        exec: &Path,
        run: &[String],
        infile: &Path,
        outfile: &Path,
    ) -> Result<()> {
        let stdout = self.run_with_input(cd, exec, run, infile)?;
        {
            // TODO better name
            let mut file = self.create_file_if_nonexistent(outfile, 0o644)?;
            // TODO write arbitrary byte sequences
            self.write_bytes_to_file(&mut file, &stdout)?;
        }
        Ok(())
    }
}

pub struct RunUtilImpl;

impl RunUtilExt for RunUtilImpl {}
impl IoUtilExt for RunUtilImpl {}
