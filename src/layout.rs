use std::str::FromStr;

#[derive(Debug)]
pub(crate) enum Layout {
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
