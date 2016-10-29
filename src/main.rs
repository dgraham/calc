extern crate calc;
extern crate getopts;

use std::env;
use std::process::exit;

use calc::{dot, eval, parse};
use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("d", "dot", "Output Graphviz dot notation");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };

    if matches.opt_present("d") {
        match parse(&matches.free.join(" ")) {
            Ok(node) => println!("{}", dot(node)),
            Err(e) => {
                println!("{}", e);
                exit(1);
            }
        }
    } else {
        match eval(&matches.free.join(" ")) {
            Ok(value) => println!("{}", value),
            Err(e) => {
                println!("{}", e);
                exit(1);
            }
        }
    }
}
