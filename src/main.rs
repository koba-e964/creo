#[macro_use]
extern crate clap;
use clap::Command as ClapCommand;
use creo::cmd::{add, all, check, gen, init, refgen, test, val, Command};
use creo::entity::project::ProjectImpl;

fn main() {
    let mut commands = [
        &mut add::AddCommand {
            project: ProjectImpl,
        } as &mut dyn Command,
        &mut all::AllCommand {
            project: ProjectImpl,
        },
        &mut init::InitCommand,
        &mut check::CheckCommand {
            project: ProjectImpl,
        },
        &mut gen::GenCommand {
            project: ProjectImpl,
        },
        &mut refgen::RefGenCommand {
            project: ProjectImpl,
        },
        &mut test::TestCommand {
            project: ProjectImpl,
        },
        &mut val::ValCommand {
            project: ProjectImpl,
        },
    ];

    let mut app = ClapCommand::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg_required_else_help(true);
    for cmd in &commands {
        app = app.subcommand(cmd.get_subcommand());
    }
    let matches = app.get_matches();
    for cmd in &mut commands {
        if let Some(()) = cmd.check(&matches) {
            return;
        }
    }
}
