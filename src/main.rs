mod error;

use std::io::{self, Read};
use std::process;

use url;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Options {
  cmd: String,
  subcmds: Vec<String>,
}

fn main() {
  match cmd() {
    Ok(_)     => {},
    Err(err)  => {
      println!("*** {}", err);
      process::exit(1);
    },
  };
}

fn cmd() -> Result<(), error::Error> {
  let opts = Options::from_args();
  match opts.cmd.as_str() {
    "resolve" | "res" => resolve(&opts),
    _                 => Err(error::Error::NoSuchCommand(opts.cmd)),
  }
}

fn resolve(opts: &Options) -> Result<(), error::Error> {
  let mut buf = String::new();
  let (base, rel) = match opts.subcmds.len() {
    2 => (&opts.subcmds[0], &opts.subcmds[1]),
    1 => {
      io::stdin().read_to_string(&mut buf);
      (&buf, &opts.subcmds[0])
    },
    _ => return Err(error::Error::InvalidArgument("Expected: <base> <relative>, or: STDIN <relative>".to_string())),
  };
  
  let base = url::Url::parse(base)?;
  let res = base.join(rel)?;
  println!("{}", res);
  
  Ok(())
}
