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
  #[clap(about="Resolve a relative URL against an absolute base")]
  Resolve(ResolveOptions),
  #[clap(about="Trim components from the end of a URL's path")]
  Trim(TrimOptions),
}

#[derive(Args, Debug)]
struct ResolveOptions {
  #[clap(long)]
  base: Option<String>,
  url: String,
}

#[derive(Args, Debug)]
struct TrimOptions {
  #[clap(long, short='n')]
  count: i32,
  url: String,
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
    Command::Trim(sub)    => trim(&opts, &sub),
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
  let resolved = base.join(&cmd.url)?;
  println!("{}", resolved);
  
  Ok(())
}

fn trim(_: &Options, cmd: &TrimOptions) -> Result<(), error::Error> {
  let mut base = url::Url::parse(&cmd.url)?;
  let mut segs = base.path_segments().ok_or_else(|| error::Error::InvalidArgument(format!("URL has no path: {}", &cmd.url)))?;
  let mut trim: Vec<&str> = Vec::new();
  
  loop {
    if let Some(seg) = segs.next() {
      trim.push(seg);
    }else{
      break;
    }
  }
  
  for _ in 0..cmd.count {
    match trim.pop() {
      Some(_) => {},
      None => break,
    };
  }
  
  base.set_path(&trim.join("/"));
  println!("{}", base);
  
  Ok(())
}
