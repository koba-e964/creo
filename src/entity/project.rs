use path_clean::PathClean;
use std::io::Result;
use std::path::Path;

use crate::entity::config::CreoConfig;
use crate::io_util::{IoUtil, IoUtilExt};
use crate::run_util::{RunUtil, RunUtilExt};

/// A trait that provides functions to handle a project directory.
pub trait Project {
    fn gen(&mut self, proj_dir: &str) -> Result<()>;
}

pub trait ProjectExt: IoUtil + RunUtil {}

impl<T: ProjectExt> Project for T {
    fn gen(&mut self, proj: &str) -> Result<()> {
        let proj = Path::new(proj);

        // Read the config file
        let config_filepath = proj.join("creo.toml");
        let mut file = self.open_file_for_read(&config_filepath)?;
        let content = self.read_from_file(&mut file)?;
        let config: CreoConfig = toml::from_str(&content).unwrap();
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

    struct MockProject;
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
            "#
            .to_string())
        }
        fn to_absolute(&self, _path: &Path) -> Result<PathBuf> {
            Ok("gen-absolute".into())
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
    }
    impl ProjectExt for MockProject {}

    #[test]
    fn gen_project_works() {
        let mut project = MockProject;
        project.gen(".").unwrap();
    }
}
