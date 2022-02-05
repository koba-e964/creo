use clap::{App, Arg, ArgMatches};

use super::Command;
use crate::entity::project::Project;

const TEST_COMMAND: &str = "test";

pub struct TestCommand<P> {
    pub project: P,
}

impl<P: Project> Command for TestCommand<P> {
    fn get_subcommand<'a>(&self) -> App<'a> {
        App::new(TEST_COMMAND).about("test all solutions").arg(
            Arg::new("PROJECT")
                .help("Project directory")
                .required(true)
                .index(1),
        )
    }
    fn check(&mut self, matches: &ArgMatches) -> Option<()> {
        let matches = matches.subcommand_matches(TEST_COMMAND)?;
        let proj_dir = matches.value_of("PROJECT").unwrap();
        if let Err(e) = self.project.test(proj_dir) {
            eprintln!("Error: {}", e);
            panic!("error: e = {:?}", e);
        }
        Some(())
    }
}
