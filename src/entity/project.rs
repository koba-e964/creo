use path_clean::PathClean;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

use crate::entity::config::CreoConfig;
use crate::entity::testcase::TestcaseConfig;
use crate::io_util::{IoUtil, IoUtilExt};
use crate::run_util::{RunUtil, RunUtilExt};

/// A trait that provides functions to handle a project directory.
pub trait Project {
    /// Generate input files from a generator.
    #[allow(unused)]
    fn gen(&mut self, proj_dir: &str) -> Result<()> {
        unreachable!();
    }
    /// Generate output files from input files and a reference solution.
    #[allow(unused)]
    fn refgen(&mut self, proj_dir: &str) -> Result<()> {
        unreachable!();
    }
}

pub trait ProjectExt: IoUtil + RunUtil {
    fn read_config(&mut self, proj: &Path) -> Result<CreoConfig> {
        // Read the config file
        let config_filepath = proj.join("creo.toml");
        let mut file = self.open_file_for_read(&config_filepath)?;
        let content = self.read_from_file(&mut file)?;
        // TODO: better error handling (user-defined error type probably helps)
        let config: CreoConfig = toml::from_str(&content).unwrap();

        Ok(config)
    }
}

impl<T: ProjectExt> Project for T {
    fn gen(&mut self, proj: &str) -> Result<()> {
        let proj = Path::new(proj);

        // Read the config file
        let config = self.read_config(proj)?;
        let lang_configs = config.languages;
        for gen in config.generators {
            let src = proj.join(&gen.path);
            let cd = src.join("..").clean();
            let cd = self.to_absolute(&cd)?;
            let lang_config = lang_configs
                .iter()
                .find(|&c| c.language_name == gen.language_name);
            if let Some(x) = lang_config {
                let outpath = self.compile(&cd, &self.to_absolute(&src)?, &x.compile)?;
                self.run_once(&cd, &outpath, &x.run)?;
            } else {
                eprintln!("warning");
                let e = Error::new(
                    ErrorKind::Other,
                    format!("language not found: {}", gen.language_name),
                );
                return Err(e);
            }
        }
        Ok(())
    }
    fn refgen(&mut self, proj_dir: &str) -> Result<()> {
        let proj_dir = Path::new(proj_dir);

        // Read the config file
        let config = self.read_config(proj_dir)?;
        let lang_configs = config.languages;
        let TestcaseConfig { indir, outdir } = config.testcase_config;
        let indir = PathBuf::from(indir);
        let outdir = PathBuf::from(outdir);

        // Do we have exactly one model solution?
        let model_solution_count = config
            .solutions
            .iter()
            .filter(|&solution| solution.is_reference_solution)
            .count();
        if model_solution_count != 1 {
            let e = Error::new(
                ErrorKind::Other,
                format!("#model solutions is not one: {}", model_solution_count),
            );
            return Err(e);
        }

        for sol in config.solutions {
            if !sol.is_reference_solution {
                continue;
            }
            let src = proj_dir.join(&sol.path);
            let cd = src.join("..").clean();
            let cd = self.to_absolute(&cd)?;
            let lang_config = lang_configs
                .iter()
                .find(|&c| c.language_name == sol.language_name);
            if let Some(x) = lang_config {
                let outpath = self.compile(&cd, &self.to_absolute(&src)?, &x.compile)?;
                // For all files in `indir`, generate the counterpart in `outdir`.
                for infile in self.list_dir(&indir)? {
                    eprintln!("Generating {}", infile.to_str().unwrap());
                    let outfile = outdir.join(&infile);
                    self.run_pipe(&cd, &outpath, &x.run, &infile, &outfile)?;
                }
            } else {
                eprintln!("warning");
                let e = Error::new(
                    ErrorKind::Other,
                    format!("language not found: {}", sol.language_name),
                );
                return Err(e);
            }
        }
        Ok(())
    }
}

pub struct ProjectImpl;

impl IoUtilExt for ProjectImpl {}
impl RunUtilExt for ProjectImpl {}
impl ProjectExt for ProjectImpl {}

mod tests {
    use std::path::PathBuf;

    use super::*;

    struct MockProject {
        processed: Vec<String>,
    }
    impl IoUtil for MockProject {
        fn open_file_for_read(&self, _filepath: &Path) -> Result<Box<dyn std::io::Read>> {
            Ok(Box::new(b"don't care" as &[u8]))
        }
        fn read_from_file(&self, _file: &mut dyn std::io::Read) -> Result<String> {
            Ok(r#"
[[generators]]
language_name = "C++"
path = "gen.cpp"

[[languages]]
language_name = "C++"
target_ext = ".cpp"
compile = ["g++", "-O2", "-std=gnu++11", "-o", "$OUT", "$IN"]
run = ["./a.out"]

[[solutions]]
path = "sol.cpp"
language_name = "C++"
is_reference_solution = true
"#
            .to_string())
        }
        fn to_absolute(&self, _path: &Path) -> Result<PathBuf> {
            Ok("gen-absolute".into())
        }
        fn list_dir(&self, _path: &Path) -> Result<Vec<PathBuf>> {
            Ok(vec!["a".into(), "b".into()])
        }
    }
    impl RunUtil for MockProject {
        fn compile(&mut self, _cd: &Path, src: &Path, _compile: &[String]) -> Result<PathBuf> {
            assert_eq!(src, PathBuf::from("gen-absolute"));
            Ok("outpath".into())
        }
        fn run_once(&mut self, _cd: &Path, exec: &Path, _run: &[String]) -> Result<()> {
            assert_eq!(exec, PathBuf::from("outpath"));
            Ok(())
        }
        fn run_pipe(
            &mut self,
            _cd: &Path,
            _exec: &Path,
            _run: &[String],
            infile: &Path,
            _outfile: &Path,
        ) -> Result<()> {
            self.processed.push(infile.to_str().unwrap().to_owned());
            Ok(())
        }
    }
    impl ProjectExt for MockProject {}

    #[test]
    fn gen_project_works() {
        let mut project = MockProject { processed: vec![] };
        project.gen(".").unwrap();
    }

    #[test]
    fn refgen_project_works() {
        let mut project = MockProject { processed: vec![] };
        project.refgen(".").unwrap();
        assert_eq!(project.processed, vec!["a".to_owned(), "b".to_owned()]);
    }
}
