use super::Command;
use clap::{Arg, ArgMatches, Command as ClapCommand};

use crate::entity::project::Project;

pub struct CheckCommand<P> {
    pub project: P,
}

impl<P: Project> Command for CheckCommand<P> {
    fn get_subcommand(&self) -> ClapCommand {
        ClapCommand::new("check").about("check creo.toml").arg(
            Arg::new("PROJECT")
                .help("Project directory")
                .required(true)
                .index(1),
        )
    }
    fn check(&mut self, matches: &ArgMatches) -> Option<()> {
        let matches = matches.subcommand_matches("check")?;
        let proj_dir = matches.get_one::<String>("PROJECT").unwrap();
        self.project.check(proj_dir).unwrap();
        Some(())
    }
}
