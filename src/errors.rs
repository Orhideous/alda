#[derive(Debug)]
pub(crate) enum Error {
    Io(std::io::Error),
    SerDe(serde_bencode::Error),
    Traverse(walkdir::Error),
    Unknown,
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self { Error::Io(err) }
}

impl From<serde_bencode::Error> for Error {
    fn from(err: serde_bencode::Error) -> Self { Error::SerDe(err) }
}

impl From<walkdir::Error> for Error {
    fn from(err: walkdir::Error) -> Self { Error::Traverse(err) }
}
