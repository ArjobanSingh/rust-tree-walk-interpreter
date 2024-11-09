use std::{env, process};

use tree_walk_interpreter::lox;

fn main() {
    if let Err(e) = lox(env::args()) {
        eprintln!("Application Error: {e}");
        process::exit(64)
    }
}
