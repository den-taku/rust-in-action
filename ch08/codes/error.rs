use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::net::AddrParseError;
use std::net::Ipv6Addr;

#[derive(Debug)]
enum UpstreamError {
    IO(io::Error),
    Parsing(AddrParseError),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl error::Error for UpstreamError {}

impl From<io::Error> for UpstreamError {
    fn from(error: io::Error) -> Self {
        Self::IO(error)
    }
}

impl From<AddrParseError> for UpstreamError {
    fn from(error: AddrParseError) -> Self {
        Self::Parsing(error)
    }
}

fn main() -> Result<(), UpstreamError> {
    let _f = File::open("invisible.txt")?;

    let _localhost = "::1".parse::<Ipv6Addr>()?;
    Ok(())
}
