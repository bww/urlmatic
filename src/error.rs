use std::io;
use std::fmt;
use std::num;

use url;

#[derive(Debug)]
pub enum Error {
  NoSuchCommand(String),
  InvalidArgument(String),
  MissingArgument(String),
  ParseIntError(num::ParseIntError),
  ParseUrlError(url::ParseError),
  IOError(io::Error),
}

impl From<std::num::ParseIntError> for Error {
  fn from(err: std::num::ParseIntError) -> Self {
    Self::ParseIntError(err)
  }
}

impl From<url::ParseError> for Error {
  fn from(err: url::ParseError) -> Self {
    Self::ParseUrlError(err)
  }
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Self {
    Self::IOError(err)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::NoSuchCommand(msg) => write!(f, "No such command: {}", msg),
      Self::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
      Self::MissingArgument(msg) => write!(f, "Missing argument: {}", msg),
      Self::ParseIntError(err) => err.fmt(f),
      Self::ParseUrlError(err) => err.fmt(f),
      Self::IOError(err) => err.fmt(f),
    }
  }
}
