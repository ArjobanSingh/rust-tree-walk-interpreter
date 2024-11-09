use std::{error::Error, io};

pub struct Lox;

impl Lox {
    pub fn begin(mut args: impl ExactSizeIterator<Item = String>) -> Result<(), Box<dyn Error>> {
        if args.len() > 2 {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Usage: jlox [script]",
            )));
        }

        args.next();

        match args.next() {
            Some(file_path) => run_file(file_path),
            None => run_prompt(),
        }

        Ok(())
    }
}

fn run_file(file_path: String) {
    println!("Path is: {file_path}");
}

fn run_prompt() {
    println!("No path");
}
