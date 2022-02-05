use clap::{App, Arg, ArgMatches};

use super::Command;
use crate::entity::project::Project;

const GEN_COMMAND: &str = "gen";

pub struct GenCommand<P> {
    pub project: P,
}

impl<P: Project> Command for GenCommand<P> {
    fn get_subcommand<'a>(&self) -> App<'a> {
        App::new(GEN_COMMAND)
            .about("generate testcases (input)")
            .arg(
                Arg::new("PROJECT")
                    .help("Project directory")
                    .required(true)
                    .index(1),
            )
    }
    fn check(&mut self, matches: &ArgMatches) -> Option<()> {
        let matches = matches.subcommand_matches(GEN_COMMAND)?;
        let proj_dir = matches.value_of("PROJECT").unwrap();
        self.project.gen(proj_dir).unwrap();
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
    }
    #[test]
    fn gen_command_positive() {
        let mut gen_command = GenCommand {
            project: MockProject,
        };
        let command = vec!["problem-creator", "gen", "project_dir"];
        let matches = App::new("problem-creator")
            .subcommand(gen_command.get_subcommand())
            .get_matches_from(command);
        assert_eq!(gen_command.check(&matches), Some(()));
    }

    #[test]
    fn gen_command_negative() {
        let mut gen_command = GenCommand {
            project: MockProject,
        };

        // unknown arguments
        let command = vec!["problem-creator", "gen", "project_dir", "--wa"];
        let matches = App::new("problem-creator")
            .subcommand(gen_command.get_subcommand())
            .try_get_matches_from(command);
        assert_eq!(
            matches.err().map(|x| x.kind),
            Some(ErrorKind::UnknownArgument),
        );

        // not `gen`
        let command = vec!["problem-creator", "test", "project_dir"];
        let matches = App::new("problem-creator")
            .subcommand(gen_command.get_subcommand())
            .subcommand(App::new("test").arg(Arg::new("PROJECT").required(true).index(1)))
            .get_matches_from(command);
        assert_eq!(gen_command.check(&matches), None);
    }
}
