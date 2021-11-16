#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate clap;
extern crate colorful;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use anyhow::Result;
use clap::Parser;
use env_logger::Builder;
use log::LevelFilter;

use crate::layout::Layout;
use crate::opts::{Opts, SubCommand};

mod actions;
mod errors;
mod layout;
mod opts;
mod torrent;

fn init_logging() {
    let mut builder = Builder::new();
    builder.filter_level(LevelFilter::Debug);
    builder.format_timestamp(None);
    builder.format_target(false);
    builder.parse_env("ALDA_LOG");
    builder.init();
}

fn main() -> Result<()> {
    init_logging();

    let opts: Opts = Opts::parse();
    trace!("{:#?}", opts);

    match opts.subcommand {
        SubCommand::Inspect(_) => actions::inspect(&opts),
        SubCommand::Relocate(ref relocate_opts) => actions::relocate(&opts, relocate_opts),
        SubCommand::Cleanup(ref cleanup_opts) => actions::cleanup(&opts, cleanup_opts),
    }
}
