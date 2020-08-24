mod ast;
mod grammar;

fn main() {
    println!("Welcome to the espressoscript compiler");
    println!("{:?}", grammar::grammar::module("1+4"));
}
