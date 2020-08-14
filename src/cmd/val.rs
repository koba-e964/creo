use clap::{App, Arg, ArgMatches, SubCommand};

use super::Command;
use crate::entity::project::Project;

const VAL_COMMAND: &str = "val";

pub struct ValCommand<P> {
    pub project: P,
}

impl<P: Project> Command for ValCommand<P> {
    fn get_subcommand<'b, 'a: 'b>(&self) -> App<'a, 'b> {
        SubCommand::with_name(VAL_COMMAND)
            .about("validate testcases (input)")
            .arg(
                Arg::with_name("PROJECT")
                    .help("Project directory")
                    .required(true)
                    .index(1),
            )
    }
    fn check(&mut self, matches: &ArgMatches) -> Option<()> {
        let matches = matches.subcommand_matches(VAL_COMMAND)?;
        let proj_dir = matches.value_of("PROJECT").unwrap();
        self.project.val(proj_dir).unwrap();
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::error::Result;
    use clap::{App, ErrorKind};

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
        let matches = App::new("problem-creator")
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
        let matches = App::new("problem-creator")
            .subcommand(val_command.get_subcommand())
            .get_matches_from_safe(command);
        assert_eq!(
            matches.err().map(|x| x.kind),
            Some(ErrorKind::UnknownArgument),
        );

        // not `val`
        let command = vec!["problem-creator", "test", "project_dir"];
        let matches = App::new("problem-creator")
            .subcommand(val_command.get_subcommand())
            .subcommand(
                SubCommand::with_name("test")
                    .arg(Arg::with_name("PROJECT").required(true).index(1)),
            )
            .get_matches_from(command);
        assert_eq!(val_command.check(&matches), None);
    }
}
