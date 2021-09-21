use structopt::StructOpt;
use std::fs::File;
use std::io::{self, BufRead};

use lang::compiler::compile;

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    file: std::path::PathBuf,
}

fn main() {
  let args = Cli::from_args();

  match File::open(&args.file) {
    Ok(file) => compile(io::BufReader::new(file).lines()),
    Err(err) => {
      println!("{}: {}", err, args.file.to_str().unwrap());
      std::process::exit(1);
    }
  }
}
