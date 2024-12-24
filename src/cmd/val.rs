use clap::{Arg, ArgMatches, Command as ClapCommand};

use super::Command;
use crate::entity::project::Project;

const VAL_COMMAND: &str = "val";

pub struct ValCommand<P> {
    pub project: P,
}

impl<P: Project> Command for ValCommand<P> {
    fn get_subcommand(&self) -> ClapCommand {
        ClapCommand::new(VAL_COMMAND)
            .about("validate testcases (input)")
            .arg(
                Arg::new("PROJECT")
                    .help("Project directory")
                    .required(true)
                    .index(1),
            )
    }
    fn check(&mut self, matches: &ArgMatches) -> Option<()> {
        let matches = matches.subcommand_matches(VAL_COMMAND)?;
        let proj_dir = matches.get_one::<String>("PROJECT").unwrap();
        self.project.val(proj_dir).unwrap();
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::error::Result;
    use clap::{error::ErrorKind, Command as ClapCommand};

    struct MockProject;
    impl Project for MockProject {
        fn val(&mut self, _proj_dir: &str) -> Result<()> {
            Ok(())
        }
    }
    #[test]
    fn val_command_positive() {
        let mut val_command = ValCommand {
            project: MockProject,
        };
        let command = vec!["problem-creator", "val", "project_dir"];
        let matches = ClapCommand::new("problem-creator")
            .subcommand(val_command.get_subcommand())
            .get_matches_from(command);
        assert_eq!(val_command.check(&matches), Some(()));
    }

    #[test]
    fn val_command_negative() {
        let mut val_command = ValCommand {
            project: MockProject,
        };

        // unknown arguments
        let command = vec!["problem-creator", "val", "project_dir", "--wa"];
        let matches = ClapCommand::new("problem-creator")
            .subcommand(val_command.get_subcommand())
            .try_get_matches_from(command);
        assert_eq!(
            matches.err().map(|x| x.kind()),
            Some(ErrorKind::UnknownArgument),
        );

        // not `val`
        let command = vec!["problem-creator", "test", "project_dir"];
        let matches = ClapCommand::new("problem-creator")
            .subcommand(val_command.get_subcommand())
            .subcommand(ClapCommand::new("test").arg(Arg::new("PROJECT").required(true).index(1)))
            .get_matches_from(command);
        assert_eq!(val_command.check(&matches), None);
    }
}
