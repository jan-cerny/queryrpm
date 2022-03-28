extern crate clap;

use clap::Parser;
use queryrpm::utils;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// RPM Package name
    name: String,
}

fn main() {
    let args = Args::parse();
    match utils::get_nvr(&args.name) {
        Some(nvr) => println!("{}", nvr),
        None => println!("Package {} was not found.", args.name),
    };
}
