use clap::{App, Arg, ArgMatches, SubCommand};

use super::Command;
use crate::entity::project::Project;

const REFGEN_COMMAND: &str = "refgen";
const SKIP_IN: &str = "SKIP_IN";
const SKIP_IN_LONG_ARG: &str = "skip-in";

pub struct RefGenCommand<P> {
    pub project: P,
}

impl<P: Project> Command for RefGenCommand<P> {
    fn get_subcommand<'b, 'a: 'b>(&self) -> App<'a, 'b> {
        SubCommand::with_name(REFGEN_COMMAND)
            .about("generate test output from a model solution")
            .arg(
                Arg::with_name(SKIP_IN)
                    .long(SKIP_IN_LONG_ARG)
                    .takes_value(false),
            )
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
        let skip_in = matches.is_present(SKIP_IN);
        if skip_in {
            println!(
                "Skipped generating input files (reason: option --{} was given)",
                SKIP_IN_LONG_ARG,
            );
        } else {
            self.project.gen(proj_dir).unwrap();
        }
        self.project.refgen(proj_dir).unwrap();
        Some(())
    }
}

#[cfg(test)]
mod tests {}
