use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;

#[derive(Debug)]
enum Layout {
    Simple,
    Nested,
}

impl FromStr for Layout {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "simple" => Ok(Layout::Simple),
            "nested" => Ok(Layout::Nested),
            _ => Err("Unknown layout"),
        }
    }
}

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
    let opts: Opts = Opts::parse();
    println!("{:#?}", opts);
}
