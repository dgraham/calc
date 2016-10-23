extern crate calc;

use std::env;
use std::process::exit;

use calc::eval;

fn main() {
    let tokens: Vec<String> = env::args().skip(1).collect();
    match eval(&tokens.join(" ")) {
        Ok(value) => println!("{}", value),
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }
}
