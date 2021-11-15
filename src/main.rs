#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use std::path::PathBuf;

use clap::Parser;
use anyhow::Result;

use crate::errors::AldaError;
use crate::layout::Layout;

mod errors;
mod fetch;
mod layout;
mod torrent;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Parser, Debug)]
#[clap(about, version)]
struct Opts {
    /// Directory with torrent files
    #[clap(short, long, parse(from_os_str))]
    torrents: PathBuf,

    /// Directory with saved content
    #[clap(short, long, parse(from_os_str))]
    source: PathBuf,

    /// Layout type
    #[clap(short, long, possible_values = ["simple", "nested"], default_value = "simple")]
    layout: Layout,

    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Inspect(Inspect),
    Move(Move),
    Cleanup(Cleanup),
}

/// Look for orphaned files without torrents
#[derive(Parser, Debug)]
struct Inspect {}

/// Move orphaned files to separate directory
#[derive(Parser, Debug)]
struct Move {
    /// Where to move orphaned files
    #[clap(short, long)]
    destination: String,
    /// Don't ask for confirmation
    #[clap(short, long)]
    force: bool,
}

/// Delete orphaned files
#[derive(Parser, Debug)]
struct Cleanup {
    /// Don't ask for confirmation
    #[clap(short, long)]
    force: bool,
}

fn main() -> Result<()> {
    simple_logger::init_with_env().unwrap();
    let opts: Opts = Opts::parse();
    trace!("{:#?}", opts);

    match opts.subcommand {
        SubCommand::Inspect(_) => {
            let known_torrents = fetch::collect_downloaded_torrents(opts.source, opts.layout)?;

            warn!("Not implemented yet!");
            Ok(())
        }
        SubCommand::Move(_) => {
            warn!("Not implemented yet!");
            Ok(())
        }
        SubCommand::Cleanup(opts) => {
            warn!("Not implemented yet!");
            Ok(())
        }
    }
    // match &result {
    //     Ok(_) => {
    //         info!("Done!")
    //     }
    //     Err(Error::Io(why)) => {
    //         error!("I/O error occured: {}", why)
    //     }
    //     Err(Error::SerDe(why)) => {}
    //     Err(Error::Traverse(why)) => {
    //         error!("Can't traverse directory: {}", why)
    //     }
    //     Err(Error::Unknown) => {}
    // }
}
