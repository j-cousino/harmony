//!  
use std::path::PathBuf;

use clap::{ArgMatches, Command};
use harmony::index::Index;

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
                .about("Reports the status of a harmony managed directory")
        )
        .subcommand(
            Command::new("init")
                .about("Prepare a directory to me managed by harmony")
        ).get_matches();

        // If we gmake it here we can read any required configuration files

        // Process main command arguments

        // then execute the subcommand
        match matches.subcommand() {
            Some(("status", args)) => {
                on_status( args )
            },
            Some(("init", args)) => {
                on_init( args )
            },
            _ => unreachable!("Should never make it here if all the subcommands are handled!"),
        }
}

fn on_status( _args: &ArgMatches) -> anyhow::Result<()> {
    // first check and see if the .harmony/index file exists
    let index_path = std::path::Path::new(".harmony/index");
    if !index_path.is_file() {
        println!("fatal: not a harmony managed directory.");
        return Ok(())
    }

    // Get the baseline index
    let baseline = Index::open(".harmony/index").expect("Failed to open index");
    let curridx = Index::new(".");
    
    // quick check for changes
    if curridx != baseline {
        println!("Changes have been made!");
    } else {
        println!("No changes were detected.");
        return Ok(())
    }

    // check for file additions/deletions
    let delta = curridx.len() - baseline.len();
    if delta > 0 {
        // files were added
        for entry in curridx.iter(){
            
        }
    }
    Ok(())     
}

fn on_init( _args: &ArgMatches ) -> anyhow::Result<()> {
    // Create base directory
    let mut base_path = PathBuf::new();
    base_path.push(".harmony");
    std::fs::create_dir(&base_path)?;

    // Create initial index
    base_path.push("index");
    Index::new(".").save(&base_path)?;

    Ok(())
}

