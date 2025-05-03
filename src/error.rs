use std::io;
use std::fmt;
use std::num;

#[derive(Debug)]
pub enum Error {
  InvalidArgument(String),
  ParseIntError(num::ParseIntError),
  ParseUrlError(url::ParseError),
  IOError(io::Error),
  RenderError(handlebars::RenderError),
  UndefinedError,
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

impl From<handlebars::RenderError> for Error {
  fn from(err: handlebars::RenderError) -> Self {
    Self::RenderError(err)
  }
}

impl From<()> for Error {
  fn from(_: ()) -> Self {
    Self::UndefinedError
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
      Self::ParseIntError(err) => err.fmt(f),
      Self::ParseUrlError(err) => err.fmt(f),
      Self::IOError(err) => err.fmt(f),
      Self::RenderError(err) => err.fmt(f),
      Self::UndefinedError => write!(f, "Undefined error"),
    }
  }
}
