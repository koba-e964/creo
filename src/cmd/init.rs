use super::Command;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::io::{Result, Write};
use std::path::Path;

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
        initialize_project(dest).unwrap();
        Some(())
    }
}

fn initialize_project(dest: &str) -> Result<()> {
    let dest = Path::new(dest);
    // config file
    let config_filepath = dest.join("creo.toml");
    let config = crate::entity::config::CreoConfig::default();
    println!("{}", toml::to_string(&config).unwrap());
    let mut file = crate::util::create_file_if_nonexistent(&config_filepath)?;
    write!(file, "{}", toml::to_string(&config).unwrap())?;
    Ok(())
}
