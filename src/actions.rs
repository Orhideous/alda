use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::string::String;

use anyhow::Result;
use colorful::Colorful;
use log::{debug, info};
use serde_bencode::de;
use walkdir::{DirEntry, WalkDir};

use crate::layout::Layout;
use crate::opts::{Cleanup, Inspect, Opts, Relocate};
use crate::torrent::Torrent;

static TORRENT_EXTENSION: &str = ".torrent";

#[derive(Debug)]
struct InspectResult<'a> {
    /// Torrents with corresponding files
    known: HashSet<&'a String>,
    /// Files without torrents
    orphaned: HashSet<&'a String>,
    /// Torrents without files
    missed: HashSet<&'a String>,
}

fn inspect_downloaded<'a>(
    fs_paths: &'a HashSet<String>,
    tor_paths: &'a HashSet<String>,
) -> InspectResult<'a> {
    let known: HashSet<_> = fs_paths.intersection(&tor_paths).collect();
    let orphaned: HashSet<_> = fs_paths.difference(&tor_paths).collect();
    let missed: HashSet<_> = tor_paths.difference(&fs_paths).collect();

    InspectResult {
        known,
        orphaned,
        missed,
    }
}

fn collect_downloaded_torrents(data_dir: &PathBuf, layout: &Layout) -> Result<HashSet<String>> {
    let depth = match layout {
        Layout::Simple => 1,
        Layout::Nested => 2,
    };

    let found: Result<Vec<DirEntry>, _> = WalkDir::new(data_dir)
        .min_depth(depth)
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
            .map(|s| s.to_string())
            .ok_or(anyhow!(
                "Entry {:?} contains symbols beyond UTF-8 range",
                entry
            ))?;
        debug!("Added {} as data directory", path.clone().blue());
        result.insert(path);
    }
    Ok(result)
}

fn parse_torrent_name(path: PathBuf) -> Result<String> {
    let mut buffer = Vec::new();
    let mut handle = File::open(&path)?;
    handle.read_to_end(&mut buffer)?;
    let torrent = de::from_bytes::<Torrent>(&buffer)?;
    Ok(torrent.info.name)
}

fn only_torrents(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(TORRENT_EXTENSION))
        .unwrap_or(false)
}

fn collect_known_torrents(torrents_dir: &PathBuf) -> Result<HashSet<String>> {
    let torrents: Result<Vec<DirEntry>, _> = WalkDir::new(torrents_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_entry(only_torrents)
        .collect();

    let mut result: HashSet<String> = HashSet::new();

    for entry in torrents? {
        let path = entry.path();
        let torrent_name = parse_torrent_name(path.to_path_buf())?;
        debug!(
            "Added {} from {}",
            &torrent_name.clone().blue(),
            path.display().to_string().blue()
        );
        result.insert(torrent_name);
    }
    Ok(result)
}

pub(crate) fn inspect(opts: &Opts, inspect_opts: &Inspect) -> Result<()> {
    let known_torrents = collect_known_torrents(&opts.torrents)?;
    let downloaded_torrents = collect_downloaded_torrents(&opts.source, &opts.layout)?;
    let inspection = inspect_downloaded(&downloaded_torrents, &known_torrents);
    info!(
        "Collected {} downloaded torrents and {} data files/directories",
        known_torrents.len().to_string().green().bold(),
        downloaded_torrents.len().to_string().green().bold(),
    );
    info!(
        "{} torrents with corresponding files (directories)",
        inspection.known.len().to_string().green().bold()
    );
    if !inspection.orphaned.is_empty() {
        warn!(
            "{} orphaned files (directories)",
            inspection.orphaned.len().to_string().yellow().bold()
        );
        if inspect_opts.verbose {
            for file in &inspection.orphaned {
                warn!("{}", file);
            }
        }
    }
    if !inspection.missed.is_empty() {
        warn!(
            "{} torrents without data",
            inspection.missed.len().to_string().yellow().bold()
        );
        if inspect_opts.verbose {
            for torrent in &inspection.missed {
                warn!("{}", torrent);
            }
        }
    }
    if (!inspection.missed.is_empty() || !inspection.orphaned.is_empty()) && !inspect_opts.verbose {
        info!(
            "Consider re-running analysis with {} flag",
            "--verbose".green().bold()
        );
    }
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
