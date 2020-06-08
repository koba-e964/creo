#[macro_use]
extern crate clap;
use clap::App;
use creo::cmd::{check, gen, init, Command};

fn main() {
    let commands = [
        &init::InitCommand as &dyn Command,
        &check::CheckCommand,
        &gen::GenCommand,
    ];

    let mut app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!());
    for cmd in &commands {
        app = app.subcommand(cmd.get_subcommand());
    }
    let matches = app.get_matches();
    for cmd in &commands {
        if let Some(()) = cmd.check(&matches) {
            break;
        }
    }
}
