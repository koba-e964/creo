use clap::{ArgMatches, Command as ClapCommand};

pub mod add;
pub mod all;
pub mod check;
pub mod gen;
pub mod init;
pub mod refgen;
pub mod test;
pub mod val;

pub trait Command {
    fn get_subcommand(&self) -> ClapCommand;
    fn check(&mut self, matches: &ArgMatches) -> Option<()>;
}
