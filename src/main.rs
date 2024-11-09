use std::{env, process};

use tree_walk_interpreter::Lox;

fn main() {
    if let Err(e) = Lox::begin(env::args()) {
        eprintln!("{e}");
        process::exit(64)
    }
}
