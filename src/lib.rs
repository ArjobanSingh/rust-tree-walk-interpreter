use std::{
    error::Error,
    fs,
    io::{self, Write},
};

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
            Some(file_path) => run_file(file_path)?,
            None => run_prompt()?,
        }

        Ok(())
    }
}

fn run_file(file_path: String) -> Result<(), io::Error> {
    let data = fs::read_to_string(file_path)?;
    run(data);
    Ok(())
}

fn run_prompt() -> Result<(), io::Error> {
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line)?;

        // If EOF, break out of loop
        if bytes_read == 0 {
            break;
        }
        run(line);
    }

    Ok(())
}

fn run(line: String) {}
