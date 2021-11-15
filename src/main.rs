#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use anyhow::Result;
use clap::Parser;

use crate::layout::Layout;
use crate::opts::{Opts, SubCommand};

mod errors;
mod actions;
mod layout;
mod opts;
mod torrent;

fn main() -> Result<()> {
    simple_logger::init_with_env().unwrap();
    let opts: Opts = Opts::parse();
    trace!("{:#?}", opts);

    match opts.subcommand {
        SubCommand::Inspect(_) => actions::inspect(&opts),
        SubCommand::Relocate(relocate_opts) => actions::relocate(&opts, &relocate_opts),
        SubCommand::Cleanup(cleanup_opts) => actions::cleanup(&opts, &cleanup_opts),
    }
}
