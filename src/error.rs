use std::fmt;
use std::num;

use url;

#[derive(Debug, PartialEq)]
pub enum Error {
  NoSuchCommand(String),
  InvalidArgument(String),
  MissingArgument(String),
  ParseIntError(num::ParseIntError),
  ParseUrlError(url::ParseError),
}

impl From<std::num::ParseIntError> for Error {
  fn from(error: std::num::ParseIntError) -> Self {
    Self::ParseIntError(error)
  }
}

impl From<url::ParseError> for Error {
  fn from(error: url::ParseError) -> Self {
    Self::ParseUrlError(error)
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
    }
  }
}
