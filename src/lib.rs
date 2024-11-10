pub mod custom_scanner;
pub mod token;
pub mod token_type;

use std::{
    error::Error,
    fs,
    io::{self, Write}, process,
};

use custom_scanner::Scanner;

static mut HAD_ERROR: bool = false;
fn toggle_had_error(value: bool) {
    // SAFETY: There are no other threads which could be accessing `HAD_ERROR`.
    unsafe {
        HAD_ERROR = value;
    }
}

fn had_error() -> bool {
    // SAFETY: There are no other threads which could be accessing `HAD_ERROR`.
    unsafe { HAD_ERROR }
}

pub fn lox<T>(mut args: T) -> Result<(), io::Error>
where
    T: ExactSizeIterator<Item = String>,
{
    if args.len() > 2 {
        eprintln!("Usage: jlox [script]");
        process::exit(64);
    }

    args.next();

    match args.next() {
        Some(file_path) => run_file(file_path)?,
        None => run_prompt()?,
    }

    Ok(())
}

fn run_file(file_path: String) -> Result<(), io::Error> {
    let data = fs::read_to_string(file_path)?;
    run(data);

    if had_error() {
        process::exit(65);
    }
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
        run(line.trim_end().to_string());
        toggle_had_error(false);
    }

    Ok(())
}

fn run(line: String) {
    let mut scanner = Scanner::new(line);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{token:?}");
    }
}

fn error(line: i32, message: String) {
    report(line, String::from(""), message);
}

fn report(line: i32, c_where: String, message: String) {
    eprintln!("[line {line}] Error{c_where}: {message}");
    toggle_had_error(true);
}
