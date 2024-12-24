use super::Command;
use clap::{Arg, ArgMatches, Command as ClapCommand};
use std::path::Path;

use crate::error::Result;
use crate::io_util::{IoUtil, IoUtilImpl};

pub struct InitCommand;

impl Command for InitCommand {
    fn get_subcommand(&self) -> ClapCommand {
        ClapCommand::new("init").about("initialize a project").arg(
            Arg::new("DESTINATION")
                .help("Destination directory")
                .required(true)
                .index(1),
        )
    }
    fn check(&mut self, matches: &ArgMatches) -> Option<()> {
        let matches = matches.subcommand_matches("init")?;
        let dest = matches.get_one::<String>("DESTINATION").unwrap();
        initialize_project(dest, &mut IoUtilImpl).unwrap();
        Some(())
    }
}

/// Create a project.
/// If creo.toml already exists, this function returns an error.
fn initialize_project(dest: &str, io_util: &mut dyn IoUtil) -> Result<()> {
    let dest = Path::new(dest);

    // Write to the config file
    let config_filepath = dest.join("creo.toml");
    let config = crate::entity::config::CreoConfig::default();
    println!("{}", toml::to_string(&config)?);
    let mut file = io_util.create_file_if_nonexistent(&config_filepath, 0o644)?;
    io_util.write_str_to_file(&mut file, &toml::to_string(&config)?)?;

    // Create subdirectories
    let subdirs = vec!["etc", "sol", "in", "out", "task"];
    for subdir in subdirs {
        io_util.mkdir_p(&dest.join(subdir))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::io::Write;
    struct MockIoUtil {
        dirs: HashSet<String>,
    }
    impl IoUtil for MockIoUtil {
        fn create_file_if_nonexistent(
            &mut self,
            _filepath: &Path,
            _mode: u32,
        ) -> Result<Box<dyn Write>> {
            Ok(Box::new(vec![]))
        }
        fn mkdir_p(&mut self, path: &Path) -> Result<()> {
            self.dirs.insert(path.to_str().unwrap().to_owned());
            Ok(())
        }
        fn write_str_to_file(&self, _file: &mut dyn Write, _s: &str) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_initialize_project() -> Result<()> {
        let mut io_util = MockIoUtil {
            dirs: HashSet::new(),
        };
        initialize_project("aa", &mut io_util)?;
        let expected = vec!["etc", "in", "out", "sol", "task"]
            .into_iter()
            .map(|s| "aa/".to_owned() + s)
            .collect();
        assert_eq!(io_util.dirs, expected);
        Ok(())
    }
}
