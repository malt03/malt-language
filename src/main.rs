use structopt::StructOpt;
use std::fs;

use lang::compiler::compile;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
    // #[structopt(parse(from_os_str))]
    // output: std::path::PathBuf,
}

fn unwrap_or_exit<T, E: std::fmt::Display>(result: core::result::Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}

fn main() {
    let args = Cli::from_args();

    let text = unwrap_or_exit(fs::read_to_string(&args.input));
    // let writer = io::BufWriter::new(unwrap_or_exit(fs::File::create(&args.output)));
    unwrap_or_exit(compile(&text));
}
