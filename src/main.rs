#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use std::path::PathBuf;

use clap::Parser;

use crate::layout::Layout;

mod layout;
mod torrent;
mod errors;
mod fetch;

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
    #[clap(short, long, possible_values = ["simple", "nested"])]
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

fn main() {
    simple_logger::init_with_env().unwrap();
    let opts: Opts = Opts::parse();
    info!("{:#?}", opts);
}
