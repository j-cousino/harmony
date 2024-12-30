//!  
use clap::{ArgMatches, Command};

fn main() -> anyhow::Result<()> {
    // Process the command line based on the following setup
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("status")
                .about("Reports the status of the repository")
        )
        .get_matches();

        // If we gmake it here we can read any required configuration files

        // Process main command arguments

        // then execute the subcommand
        match matches.subcommand() {
            Some(("status", args)) => {
                on_status( args )
            },
            _ => unreachable!("Should never make it here if all the subcommands are handled!"),
        }
}

fn on_status( _args: &ArgMatches) -> anyhow::Result<()> {
    
    anyhow::Ok(())
}

