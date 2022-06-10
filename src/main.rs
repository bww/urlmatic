mod error;

use std::io::{self, Read};
use std::process;
use std::collections;

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
  #[clap(about="Rewrite URL components")]
  Rewrite(RewriteOptions),
  #[clap(about="Decode URL-encoded parameter lists")]
  Decode(DecodeOptions),
  #[clap(about="Encode URL-encoded parameter lists")]
  Encode(EncodeOptions),
}

#[derive(Args, Debug)]
struct ResolveOptions {
  #[clap(long, short='b', help="The base URL to resolve against")]
  base: String,
  #[clap(help="The URL to resolve against the base; if a URL is not provided it is read from STDIN")]
  url: Option<String>,
}

#[derive(Args, Debug)]
struct TrimOptions {
  #[clap(long, short='n', help="The number of path components to remove from the end of the URL")]
  count: i32,
  #[clap(help="The URL to trim; if a URL is not provided it is read from STDIN")]
  url: Option<String>,
}

#[derive(Args, Debug)]
struct RewriteOptions {
  #[clap(long, short='s', help="Set the scheme")]
  scheme: Option<String>,
  #[clap(long, short='H', help="Set the host")]
  host: Option<String>,
  #[clap(long, short='u', help="Set the authority username")]
  username: Option<String>,
  #[clap(long, short='w', help="Set the authority password")]
  password: Option<String>,
  #[clap(long, short='p', help="Set the path")]
  path: Option<String>,
  #[clap(long, short='q', help="Set the query")]
  query: Option<String>,
  #[clap(long, short='f', help="Set the fragment")]
  fragment: Option<String>,
  #[clap(help="The URL to rewrite; if a URL is not provided it is read from STDIN")]
  url: Option<String>,
}

#[derive(Args, Debug)]
struct DecodeOptions {
  #[clap(long, short='s', name="KEY[,KEY...]", help="Select keys to print. When provided the value for each key specified is printed on its own line, in the order they are encountered. Specify repeatedly to select multiple keys.")]
  select: Option<Vec<String>>,
  #[clap(help="The query string to evaluate; if a query is not provided it is read from STDIN")]
  query: Option<String>,
}

#[derive(Args, Debug)]
struct EncodeOptions {
  #[clap(long="key", short='k', name="KEY", help="A key to encode. Each key must have a corresponding --value.")]
  keys: Vec<String>,
  #[clap(long="value", short='v', name="VALUE", help="A value to encode. Each key must have a corresponding --key.")]
  values: Vec<String>,
  #[clap(help="The key/value pairs to evaluate, provided in the form 'KEY=VALUE'")]
  pairs: Vec<String>,
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
    Command::Rewrite(sub) => rewrite(&opts, &sub),
    Command::Decode(sub)  => decode(&opts, &sub),
    Command::Encode(sub)  => encode(&opts, &sub),
  }
}

fn resolve(_: &Options, cmd: &ResolveOptions) -> Result<(), error::Error> {
  let url = match &cmd.url {
    Some(url) => url.to_owned(),
    None => {
      let mut buf = String::new();
      io::stdin().read_to_string(&mut buf)?;
      buf
    },
  };
  
  let base = url::Url::parse(&cmd.base)?;
  let resolved = base.join(&url)?;
  println!("{}", resolved);
  
  Ok(())
}

fn trim(_: &Options, cmd: &TrimOptions) -> Result<(), error::Error> {
  let url = match &cmd.url {
    Some(url) => url.to_owned(),
    None => {
      let mut buf = String::new();
      io::stdin().read_to_string(&mut buf)?;
      buf
    },
  };
  
  let mut base = url::Url::parse(&url)?;
  let mut segs = base.path_segments().ok_or_else(|| error::Error::InvalidArgument(format!("URL has no path: {}", &url)))?;
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

fn rewrite(_: &Options, cmd: &RewriteOptions) -> Result<(), error::Error> {
  let url = match &cmd.url {
    Some(url) => url.to_owned(),
    None => {
      let mut buf = String::new();
      io::stdin().read_to_string(&mut buf)?;
      buf
    },
  };
  
  let mut base = url::Url::parse(&url)?;
  if let Some(v) = &cmd.scheme {
    base.set_scheme(v);
  }
  if let Some(v) = &cmd.host {
    base.set_host(Some(v))?;
  }
  if let Some(v) = &cmd.username {
    base.set_username(v);
  }
  if let Some(v) = &cmd.password {
    base.set_password(Some(v));
  }
  if let Some(v) = &cmd.path {
    base.set_path(v);
  }
  if let Some(v) = &cmd.query {
    base.set_query(Some(v));
  }
  if let Some(v) = &cmd.fragment {
    base.set_fragment(Some(v));
  }
  
  println!("{}", base);
  Ok(())
}

fn decode(_: &Options, cmd: &DecodeOptions) -> Result<(), error::Error> {
  let select: Option<collections::HashSet<String>> = match &cmd.select {
    Some(select) => Some(collections::HashSet::from_iter(select.iter().flat_map(|e| e.split(",").map(|e| e.to_string())))),
    None => None,
  };
  
  let query = match cmd.query.to_owned() {
    Some(query) => query,
    None => {
      let mut buf = String::new();
      io::stdin().read_to_string(&mut buf)?;
      buf
    },
  };
  
  let parsed = url::form_urlencoded::parse(&query.as_bytes());
  
  let mut widest: usize = 0;
  for (k, _) in parsed {
    let n = k.chars().count();
    if n > widest {
      widest = n;
    }
  }
  
  for (k, v) in parsed {
    match &select {
      Some(select) => {
        if select.contains(k.as_ref()) {
          println!("{}", v);
        }
      },
      None => {
        let mut buf = String::new();
        let n = k.chars().count();
        let p = widest-n;
        if p > 0 {
          buf.push_str(&" ".repeat(p));
        }
        buf.push_str(&k);
        buf.push_str(": ");
        buf.push_str(&v);
        println!("{}", buf);
      },
    }
  }
  
  Ok(())
}

fn encode(_: &Options, cmd: &EncodeOptions) -> Result<(), error::Error> {
  let mut params: collections::HashMap<&str, Option<&str>> = collections::HashMap::new();
  for e in &cmd.pairs {
    match e.find("=") {
      Some(x) => params.insert(&e[..x], Some(&e[x+1..])),
      None => params.insert(&e, None),
    };
  }
  
  if cmd.keys.len() != cmd.values.len() {
    return Err(error::Error::InvalidArgument("Provided --key and --value flags are not balanced. Each --key must have a corresponding --value, which are matched up in the order they are specified.".to_string()));
  }
  for i in 0..cmd.keys.len() {
    params.insert(&cmd.keys[i], Some(&cmd.values[i]));
  }
  
  let mut enc = url::form_urlencoded::Serializer::new(String::new());
  for (k, v) in &params {
    match v {
      Some(v) => enc.append_pair(k, v),
      None => enc.append_key_only(k),
    };
  }
  
  println!("{}", enc.finish());
  Ok(())
}
