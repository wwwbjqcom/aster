use btoi;
use futures::unsync::mpsc::SendError;
use net2::TcpBuilder;
use tokio::net::TcpListener;

use std::convert::From;
use std::io;
use std::net;
use std::num;
use std::result;
use std::net::SocketAddr;

#[derive(Debug)]
pub enum Error {
    None,
    MoreData,
    NotSupport,
    BadMsg,
    BadKey,
    BadCmd,
    BadConfig,
    BadSlotsMap,
    IoError(io::Error),
    Critical,
    StrParseIntError(num::ParseIntError),
    ParseIntError(btoi::ParseIntegerError),
    SendError(SendError<::Resp>),
    AddrParseError(net::AddrParseError),
}

impl From<net::AddrParseError> for Error {
    fn from(oe: net::AddrParseError) -> Error {
        Error::AddrParseError(oe)
    }
}

impl From<SendError<::Resp>> for Error {
    fn from(oe: SendError<::Resp>) -> Error {
        Error::SendError(oe)
    }
}

impl From<io::Error> for Error {
    fn from(oe: io::Error) -> Error {
        Error::IoError(oe)
    }
}

impl From<btoi::ParseIntegerError> for Error {
    fn from(oe: btoi::ParseIntegerError) -> Error {
        Error::ParseIntError(oe)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(oe: num::ParseIntError) -> Error {
        Error::StrParseIntError(oe)
    }
}

pub type AsResult<T> = result::Result<T, Error>;

const LOWER_BEGIN: u8 = 'a' as u8;
const LOWER_END: u8 = 'z' as u8;
const UPPER_BEGIN: u8 = 'A' as u8;
const UPPER_END: u8 = 'Z' as u8;
const UPPER_TO_LOWER: u8 = 'a' as u8 - 'A' as u8;

pub fn update_to_upper(src: &mut [u8]) {
    for b in src {
        if *b < LOWER_BEGIN || *b > LOWER_END {
            continue;
        }
        *b = *b - UPPER_TO_LOWER;
    }
}

pub fn update_to_lower(src: &mut [u8]) {
    for b in src {
        if *b < UPPER_BEGIN || *b > UPPER_END {
            continue;
        }
        *b = *b + UPPER_TO_LOWER;
    }
}

#[cfg(windows)]
pub fn create_reuse_port_listener(addr: &SocketAddr) -> Result<TcpListener, std::io::Error> {
    let builder = TcpBuilder::new_v4()?;
    let std_listener = builder
        .reuse_address(true)
        .expect("os not support SO_REUSEADDR")
        .bind(addr)?
        .listen(std::i32::MAX)?;
    let hd = tokio::reactor::Handle::current();
    TcpListener::from_std(std_listener, &hd)
}

#[cfg(not(windows))]
pub fn create_reuse_port_listener(addr: &SocketAddr) -> Result<TcpListener, std::io::Error> {
    use net2::unix::UnixTcpBuilderExt;

    let builder = TcpBuilder::new_v4()?;
    let std_listener = builder
        .reuse_address(true)
        .expect("os not support SO_REUSEADDR")
        .reuse_port(true)
        .expect("os not support SO_REUSEPORT")
        .bind(addr)?
        .listen(std::i32::MAX)?;
    let hd = tokio::reactor::Handle::current();
    TcpListener::from_std(std_listener, &hd)
}
