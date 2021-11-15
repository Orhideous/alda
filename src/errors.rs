use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum AldaError {
    #[error("Failed to process file")]
    Io(#[from] io::Error),
    #[error("Failed to deserialize torrent")]
    SerDe(#[from] serde_bencode::Error),
    #[error("Failed to traverse directory")]
    Traverse(#[from] walkdir::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
