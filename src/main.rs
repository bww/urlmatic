mod error;

use url;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Options {
  cmd: String,
  subcmds: Vec<String>,
}

fn main() -> Result<(), error::Error> {
  let opts = Options::from_args();
  match opts.cmd.as_str() {
    "resolve" | "res" => resolve(&opts),
    _                 => Err(error::Error::NoSuchCommand(opts.cmd)),
  }
}

fn resolve(opts: &Options) -> Result<(), error::Error> {
  let (base, rel) = match opts.subcmds.len() {
    2 => (&opts.subcmds[0], &opts.subcmds[1]),
    _ => return Err(error::Error::InvalidArgument("Expected: <base> <relative>".to_string())),
  };
  
  let base = url::Url::parse(base)?;
  let res = base.join(rel)?;
  println!("{}", res);
  
  Ok(())
}
