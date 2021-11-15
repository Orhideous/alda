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
mod fetch;
mod layout;
mod opts;
mod torrent;

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
}
