use clap::{App, Arg, ArgMatches};

use super::Command;
use crate::entity::project::Project;

const ALL_COMMAND: &str = "all";

pub struct AllCommand<P> {
    pub project: P,
}

impl<P: Project> Command for AllCommand<P> {
    fn get_subcommand<'a>(&self) -> App<'a> {
        App::new(ALL_COMMAND).about("run all processes").arg(
            Arg::new("PROJECT")
                .help("Project directory")
                .required(true)
                .index(1),
        )
    }
    fn check(&mut self, matches: &ArgMatches) -> Option<()> {
        let matches = matches.subcommand_matches(ALL_COMMAND)?;
        let proj_dir = matches.value_of("PROJECT").unwrap();
        self.project.gen(proj_dir).unwrap();
        self.project.val(proj_dir).unwrap();
        self.project.refgen(proj_dir).unwrap();
        self.project.test(proj_dir).unwrap();
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
        fn gen(&mut self, _proj_dir: &str) -> Result<()> {
            Ok(())
        }
        fn refgen(&mut self, _proj_dir: &str) -> Result<()> {
            Ok(())
        }
        fn val(&mut self, _proj_dir: &str) -> Result<()> {
            Ok(())
        }
        fn test(&mut self, _proj_dir: &str) -> Result<()> {
            Ok(())
        }
    }
    #[test]
    fn all_command_positive() {
        let mut all_command = AllCommand {
            project: MockProject,
        };
        let command = vec!["problem-creator", "all", "project_dir"];
        let matches = App::new("problem-creator")
            .subcommand(all_command.get_subcommand())
            .get_matches_from(command);
        assert_eq!(all_command.check(&matches), Some(()));
    }

    #[test]
    fn all_command_negative() {
        let mut all_command = AllCommand {
            project: MockProject,
        };

        // unknown arguments
        let command = vec!["problem-creator", "all", "project_dir", "--wa"];
        let matches = App::new("problem-creator")
            .subcommand(all_command.get_subcommand())
            .try_get_matches_from(command);
        assert_eq!(
            matches.err().map(|x| x.kind),
            Some(ErrorKind::UnknownArgument),
        );

        // not `all`
        let command = vec!["problem-creator", "test", "project_dir"];
        let matches = App::new("problem-creator")
            .subcommand(all_command.get_subcommand())
            .subcommand(App::new("test").arg(Arg::new("PROJECT").required(true).index(1)))
            .get_matches_from(command);
        assert_eq!(all_command.check(&matches), None);
    }
}
