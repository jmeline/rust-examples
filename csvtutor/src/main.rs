// This makes the csv crate accessible to your program.
extern crate csv;

use std::error::Error;
use std::ffi::OsString;
use std::process;
use std::fs::File;
use std::env;

// The `main` function is where your program starts executing.
fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<Error>> {
    // Create a CSV parser that reads data from stdin.
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    // Loop over each record.
    for result in rdr.records() {
        // An error may occur, so abort the program in an unfriendly way.
        // We will make this more friendly later!
        let record = result?;
        if &record[2] == "38344" {
            println!("{:?}", record);
        }
    }
    println!("Done!");
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}
