use clap::{App, ArgMatches};

pub mod check;
pub mod gen;
pub mod init;

pub trait Command {
    fn get_subcommand<'b, 'a: 'b>(&self) -> App<'a, 'b>;
    fn check(&self, matches: &ArgMatches) -> Option<()>;
}
