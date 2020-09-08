use crate::error::CompileError;
use clap::Clap;
use std::fs;
use std::io::Read;

mod ast;
mod compiler;
mod error;
mod grammar;
mod typechecker;

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

fn main() -> Result<(), CompileError> {
    println!("Welcome to the EspressoScript compiler!");
    let opts: Opts = Opts::parse();
    println!("Compiling: {:?} to {:?}", &opts.input, &opts.output);

    let mut contents = String::new();
    let _ = fs::File::open(&opts.input)
        .expect(&format!("Unable to open: {}", &opts.input))
        .read_to_string(&mut contents);

    match grammar::grammar::module(&contents) {
        Ok(result) => {
            typechecker::TypeChecker::new().check_module(&result)?;
            compiler::codegen::compile_to_file(result, opts.output)?;
            Ok(())
        }
        Err(e) => Err(CompileError::from(e)),
    }
}
