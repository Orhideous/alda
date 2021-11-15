use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Result;
use log::{debug, info};
use serde_bencode::de;
use walkdir::{DirEntry, WalkDir};

use crate::layout::Layout;
use crate::opts::{Cleanup, Opts, Relocate};
use crate::torrent::Torrent;

#[derive(Debug)]
struct InspectResult<'a> {
    known: HashSet<&'a String>,
    orphaned: HashSet<&'a String>,
    missed: HashSet<&'a String>,
}

fn inspect_downloaded<'a>(
    fs_paths: &'a HashSet<String>,
    tor_paths: &'a HashSet<String>,
) -> InspectResult<'a> {
    let known: HashSet<_> = fs_paths.intersection(tor_paths).collect();
    let orphaned: HashSet<_> = fs_paths.difference(tor_paths).collect();
    let missed: HashSet<_> = tor_paths.difference(fs_paths).collect();

    InspectResult {
        known,
        orphaned,
        missed,
    }
}

fn collect_downloaded_torrents(data_dir: PathBuf, layout: Layout) -> Result<HashSet<String>> {
    let depth = match layout {
        Layout::Simple => 1,
        Layout::Nested => 2,
    };

    let found: Result<Vec<DirEntry>, _> = WalkDir::new(data_dir)
        .max_depth(depth)
        .into_iter()
        .collect();

    let mut result: HashSet<String> = HashSet::new();

    for entry in found? {
        let path = entry
            .path()
            .file_name()
            .ok_or(anyhow!("Empty filename for entry {:?}", entry))?
            .to_str()
            .unwrap();
        debug!("Added {} as downloaded torrent", path);
        result.insert(String::from(path));
    }

    info!("Collected {} downloaded torrents", result.len());
    Ok(result)
}

fn parse_torrent_name(path: PathBuf) -> Result<String> {
    let mut buffer = Vec::new();
    let mut handle = File::open(&path)?;
    handle.read_to_end(&mut buffer)?;
    let torrent = de::from_bytes::<Torrent>(&buffer)?;
    Ok(torrent.info.name)
}

fn collect_known_torrents(torrents_dir: PathBuf) -> Result<HashSet<String>> {
    let found: Result<Vec<DirEntry>, _> = WalkDir::new(torrents_dir)
        .max_depth(1)
        .into_iter()
        .collect();

    let mut result: HashSet<String> = HashSet::new();

    for entry in found? {
        let path = entry.path();
        let torrent_name = parse_torrent_name(path.to_path_buf())?;
        debug!(
            "Added {} as known torrent from {}",
            &torrent_name,
            path.display()
        );
        result.insert(torrent_name);
    }

    info!("Collected {} known torrents", result.len());
    Ok(result)
}

pub(crate) fn inspect(opts: &Opts) -> Result<()> {
    warn!("Not implemented yet!");
    Ok(())
}

pub(crate) fn relocate(opts: &Opts, relocate_opts: &Relocate) -> Result<()> {
    warn!("Not implemented yet!");
    Ok(())
}

pub(crate) fn cleanup(opts: &Opts, cleanup_opts: &Cleanup) -> Result<()> {
    warn!("Not implemented yet!");
    Ok(())
}