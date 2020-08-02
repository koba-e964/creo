use super::Command;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::io::Result;
use std::path::Path;

use crate::entity::config::CreoConfig;
use crate::io_util::{IoUtil, IoUtilImpl};

pub struct CheckCommand;

impl Command for CheckCommand {
    fn get_subcommand<'b, 'a: 'b>(&self) -> App<'a, 'b> {
        SubCommand::with_name("check").about("check creo.toml").arg(
            Arg::with_name("PROJECT")
                .help("Project directory")
                .required(true)
                .index(1),
        )
    }
    fn check(&mut self, matches: &ArgMatches) -> Option<()> {
        let matches = matches.subcommand_matches("check")?;
        let proj = matches.value_of("PROJECT").unwrap();
        check_project(proj, &mut IoUtilImpl).unwrap();
        Some(())
    }
}

fn check_project(proj: &str, io_util: &mut dyn IoUtil) -> Result<()> {
    let proj = Path::new(proj);

    // Read the config file
    let config_filepath = proj.join("creo.toml");
    let mut file = io_util.open_file_for_read(&config_filepath)?;
    let content = io_util.read_from_file(&mut file)?;
    println!("{}", content);
    let config: CreoConfig = toml::from_str(&content).unwrap();
    eprintln!("config = {:?}", config);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    struct MockIoUtil;
    impl IoUtil for MockIoUtil {
        fn create_file_if_nonexistent(&mut self, _filepath: &Path) -> Result<Box<dyn Write>> {
            unreachable!();
        }
        fn open_file_for_read(&self, _filepath: &Path) -> Result<Box<dyn std::io::Read>> {
            Ok(Box::new(b"don't care" as &[u8]))
        }
        fn mkdir_p(&mut self, _path: &Path) -> Result<()> {
            unreachable!();
        }
        fn write_str_to_file(&self, _file: &mut dyn Write, _s: &str) -> Result<()> {
            unreachable!();
        }
        fn read_from_file(&self, _file: &mut dyn std::io::Read) -> Result<String> {
            Ok("".to_string())
        }
    }
    #[test]
    fn check_project_works() {
        let mut io_util = MockIoUtil;
        check_project(".", &mut io_util).unwrap();
    }
}
