use clap::{App, Arg, ArgMatches, SubCommand};

use super::Command;
use crate::entity::project::Project;

const REFGEN_COMMAND: &str = "refgen";

pub struct RefGenCommand<P> {
    pub project: P,
}

impl<P: Project> Command for RefGenCommand<P> {
    fn get_subcommand<'b, 'a: 'b>(&self) -> App<'a, 'b> {
        SubCommand::with_name(REFGEN_COMMAND)
            .about("generate test output from a model solution")
            .arg(
                Arg::with_name("PROJECT")
                    .help("Project directory")
                    .required(true)
                    .index(1),
            )
    }
    fn check(&mut self, matches: &ArgMatches) -> Option<()> {
        let matches = matches.subcommand_matches(REFGEN_COMMAND)?;
        let proj_dir = matches.value_of("PROJECT").unwrap();
        self.project.gen(proj_dir).unwrap();
        self.project.refgen(proj_dir).unwrap();
        Some(())
    }
}

#[cfg(test)]
mod tests {}
