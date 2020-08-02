use clap::{App, Arg, ArgMatches, SubCommand};
use path_clean::PathClean;
use std::io::Result;
use std::path::Path;

use super::Command;
use crate::entity::config::CreoConfig;
use crate::io_util::{IoUtil, IoUtilImpl};
use crate::run_util::{RunUtil, RunUtilImpl};

const GEN_COMMAND: &str = "gen";

pub struct GenCommand;

impl Command for GenCommand {
    fn get_subcommand<'b, 'a: 'b>(&self) -> App<'a, 'b> {
        SubCommand::with_name(GEN_COMMAND)
            .about("generate testcases (input)")
            .arg(
                Arg::with_name("PROJECT")
                    .help("Project directory")
                    .required(true)
                    .index(1),
            )
    }
    fn check(&self, matches: &ArgMatches) -> Option<()> {
        let matches = matches.subcommand_matches(GEN_COMMAND)?;
        let proj = matches.value_of("PROJECT").unwrap();
        gen_project(
            proj,
            &mut IoUtilImpl,
            &mut RunUtilImpl {
                io_util: Box::new(IoUtilImpl),
            },
        )
        .unwrap();
        Some(())
    }
}

fn gen_project(proj: &str, io_util: &mut dyn IoUtil, run_util: &mut dyn RunUtil) -> Result<()> {
    let proj = Path::new(proj);

    // Read the config file
    let config_filepath = proj.join("creo.toml");
    let mut file = io_util.open_file_for_read(&config_filepath)?;
    let content = io_util.read_from_file(&mut file)?;
    let config: CreoConfig = toml::from_str(&content).unwrap();
    let lang_configs = config.languages;
    for gen in config.generators {
        let src = proj.join(&gen.path);
        let cd = src.join("..").clean();
        let cd = io_util.to_absolute(&cd)?;
        let lang_config = lang_configs
            .iter()
            .find(|&c| c.language_name == gen.language_name);
        if let Some(x) = lang_config {
            let outpath = run_util.compile(&cd, &io_util.to_absolute(&src)?, &x.compile)?;
            run_util.run_once(&cd, &outpath, &x.run)?;
        } else {
            eprintln!("warning");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    struct MockIoUtil;
    impl IoUtil for MockIoUtil {
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
    struct MockRunUtil;
    impl RunUtil for MockRunUtil {
        fn compile(&mut self, _cd: &Path, src: &Path, _compile: &[String]) -> Result<PathBuf> {
            assert_eq!(src, PathBuf::from("gen-absolute"));
            Ok("outpath".into())
        }
        fn run_once(&mut self, _cd: &Path, exec: &Path, _run: &[String]) -> Result<()> {
            assert_eq!(exec, PathBuf::from("outpath"));
            Ok(())
        }
    }
    #[test]
    fn gen_project_works() {
        let mut io_util = MockIoUtil;
        let mut run_util = MockRunUtil;
        gen_project(".", &mut io_util, &mut run_util).unwrap();
    }
}
