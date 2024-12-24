use clap::{Arg, ArgMatches, Command as ClapCommand};

use super::Command;
use crate::entity::project::Project;

const ADD_COMMAND: &str = "add";

pub struct AddCommand<P> {
    pub project: P,
}

impl<P: Project> Command for AddCommand<P> {
    fn get_subcommand(&self) -> ClapCommand {
        ClapCommand::new("add")
            .about("add an entity")
            .arg(
                Arg::new("PROJECT")
                    .help("Project directory")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("TYPE")
                    .help("Entity type to add")
                    .required(true)
                    .index(2),
            )
            .arg(
                Arg::new("NAME")
                    .help("The entity's name")
                    .required(true)
                    .index(3),
            )
    }
    fn check(&mut self, matches: &ArgMatches) -> Option<()> {
        let matches = matches.subcommand_matches(ADD_COMMAND)?;
        let proj_dir = matches.get_one::<String>("PROJECT").unwrap();
        let ty = matches.get_one::<String>("TYPE").unwrap();
        let name = matches.get_one::<String>("NAME").unwrap();
        self.project.add(proj_dir, ty, name).unwrap();
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
        let matches = ClapCommand::new("problem-creator")
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
        let matches = ClapCommand::new("problem-creator")
            .subcommand(add_command.get_subcommand())
            .try_get_matches_from(command);
        assert_eq!(
            matches.err().map(|x| x.kind()),
            Some(ErrorKind::UnknownArgument),
        );

        // not `add`
        let command = vec!["problem-creator", "test", "project_dir"];
        let matches = ClapCommand::new("problem-creator")
            .subcommand(add_command.get_subcommand())
            .subcommand(ClapCommand::new("test").arg(Arg::new("PROJECT").required(true).index(1)))
            .get_matches_from(command);
        assert_eq!(add_command.check(&matches), None);
    }
}
