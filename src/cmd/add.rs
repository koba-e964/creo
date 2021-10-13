use clap::{App, Arg, ArgMatches, SubCommand};

use super::Command;
use crate::entity::project::Project;

const ADD_COMMAND: &str = "add";

pub struct AddCommand<P> {
    pub project: P,
}

impl<P: Project> Command for AddCommand<P> {
    fn get_subcommand<'b, 'a: 'b>(&self) -> App<'a, 'b> {
        SubCommand::with_name("add")
            .about("add an entity")
            .arg(
                Arg::with_name("PROJECT")
                    .help("Project directory")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("TYPE")
                    .help("Entity type to add")
                    .required(true)
                    .index(2),
            )
            .arg(
                Arg::with_name("NAME")
                    .help("The entity's name")
                    .required(true)
                    .index(3),
            )
    }
    fn check(&mut self, matches: &ArgMatches) -> Option<()> {
        let matches = matches.subcommand_matches(ADD_COMMAND)?;
        let proj_dir = matches.value_of("PROJECT").unwrap();
        let ty = matches.value_of("TYPE").unwrap();
        let name = matches.value_of("NAME").unwrap();
        self.project.add(proj_dir, ty, name).unwrap();
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
        fn add(&mut self, _proj_dir: &str, _ty: &str, _name: &str) -> Result<()> {
            Ok(())
        }
    }
    #[test]
    fn add_command_positive() {
        let mut add_command = AddCommand {
            project: MockProject,
        };
        let command = vec![
            "problem-creator",
            "add",
            "project_dir",
            "sol",
            "sol-koba.cpp",
        ];
        let matches = App::new("problem-creator")
            .subcommand(add_command.get_subcommand())
            .get_matches_from(command);
        assert_eq!(add_command.check(&matches), Some(()));
    }

    #[test]
    fn add_command_negative() {
        let mut add_command = AddCommand {
            project: MockProject,
        };

        // unknown arguments
        let command = vec!["problem-creator", "add", "project_dir", "--wa"];
        let matches = App::new("problem-creator")
            .subcommand(add_command.get_subcommand())
            .get_matches_from_safe(command);
        assert_eq!(
            matches.err().map(|x| x.kind),
            Some(ErrorKind::UnknownArgument),
        );

        // not `add`
        let command = vec!["problem-creator", "test", "project_dir"];
        let matches = App::new("problem-creator")
            .subcommand(add_command.get_subcommand())
            .subcommand(
                SubCommand::with_name("test")
                    .arg(Arg::with_name("PROJECT").required(true).index(1)),
            )
            .get_matches_from(command);
        assert_eq!(add_command.check(&matches), None);
    }
}
