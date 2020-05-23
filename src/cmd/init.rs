use super::Command;
use clap::{App, Arg, ArgMatches, SubCommand};

pub struct InitCommand;

impl Command for InitCommand {
    fn get_subcommand<'b, 'a: 'b>(&self) -> App<'a, 'b> {
        SubCommand::with_name("init")
            .about("initialize a project")
            .arg(
                Arg::with_name("DESTINATION")
                    .help("Destination directory")
                    .required(true)
                    .index(1),
            )
    }
    fn check(&self, matches: &ArgMatches) -> Option<()> {
        let matches = matches.subcommand_matches("init")?;
        let dest = matches.value_of("DESTINATION").unwrap();
        println!("dest = {}", dest);
        Some(())
    }
}
