mod error;

use std::io::{self, Read};
use std::process;

use url;
use clap::{Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Options {
  #[clap(long)]
  debug: bool,
  #[clap(subcommand)]
  command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
  #[clap(author, version, about, long_about = None)]
  Resolve(ResolveOptions),
}

#[derive(Args, Debug)]
struct ResolveOptions {
  #[clap(long)]
  base: Option<String>,
  #[clap(long)]
  relative: String,
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
  let opts = Options::parse();
  match &opts.command {
    Command::Resolve(sub) => resolve(&opts, &sub),
  }
}

fn resolve(_: &Options, cmd: &ResolveOptions) -> Result<(), error::Error> {
  let base = match cmd.base.to_owned() {
    Some(base) => base,
    None => {
      let mut buf = String::new();
      io::stdin().read_to_string(&mut buf)?;
      buf
    },
  };
  
  let base = url::Url::parse(&base)?;
  let resolved = base.join(&cmd.relative)?;
  println!("{}", resolved);
  
  Ok(())
}
