mod grammar;

fn main() {
    println!("Welcome to the espressoscript compiler");
    println!("{:?}", grammar::grammar::arithmetic("1+a"));
}
