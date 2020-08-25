use clap::Clap;
use std::fs;
use std::io::Read;

mod ast;
mod grammar;

/// The EspressoScript compiler
#[derive(Clap)]
#[clap(
    version = "pre-alpha",
    author = "Andrew <3865404+andrewgrz@users.noreply.github.com>"
)]
struct Opts {
    /// Runs the file.
    input: String,
    /// Generates the output to here
    output: String,
}

fn main() {
    println!("Welcome to the EspressoScript compiler!");
    let opts: Opts = Opts::parse();

    let mut contents = String::new();
    let _ = fs::File::open(&opts.input)
        .expect(&format!("Unable to open: {}", &opts.input))
        .read_to_string(&mut contents);

    println!("{:#?}", grammar::grammar::module(&contents));
}
