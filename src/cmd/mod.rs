use clap::{App, ArgMatches};

pub mod all;
pub mod check;
pub mod gen;
pub mod init;
pub mod refgen;
pub mod test;
pub mod val;

pub trait Command {
    fn get_subcommand<'b, 'a: 'b>(&self) -> App<'a, 'b>;
    fn check(&mut self, matches: &ArgMatches) -> Option<()>;
}
