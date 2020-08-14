use super::Command;
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::entity::project::Project;

pub struct CheckCommand<P> {
    pub project: P,
}

impl<P: Project> Command for CheckCommand<P> {
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
        let proj_dir = matches.value_of("PROJECT").unwrap();
        self.project.check(proj_dir).unwrap();
        Some(())
    }
}
