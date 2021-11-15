use std::path::PathBuf;

use crate::Layout;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Parser, Debug)]
#[clap(about, version)]
pub(crate) struct Opts {
    /// Directory with torrent files
    #[clap(short, long, parse(from_os_str))]
    pub(crate) torrents: PathBuf,

    /// Directory with saved content
    #[clap(short, long, parse(from_os_str))]
    pub(crate) source: PathBuf,

    /// Layout type
    #[clap(short, long, possible_values = ["simple", "nested"], default_value = "simple")]
    pub(crate) layout: Layout,

    #[clap(subcommand)]
    pub(crate) subcommand: SubCommand,
}

#[derive(Parser, Debug)]
pub(crate) enum SubCommand {
    Inspect(Inspect),
    Relocate(Relocate),
    Cleanup(Cleanup),
}

/// Look for orphaned files without torrents
#[derive(Parser, Debug)]
pub(crate) struct Inspect {}

/// Relocate orphaned files to separate directory
#[derive(Parser, Debug)]
pub(crate) struct Relocate {
    /// Where to move orphaned files
    #[clap(short, long)]
    pub(crate) destination: String,
    /// Don't ask for confirmation
    #[clap(short, long)]
    pub(crate) force: bool,
}

/// Delete orphaned files
#[derive(Parser, Debug)]
pub(crate) struct Cleanup {
    /// Don't ask for confirmation
    #[clap(short, long)]
    pub(crate) force: bool,
}
