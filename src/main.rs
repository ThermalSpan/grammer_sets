extern crate clap;
#[macro_use] extern crate nom; 

mod parser;
mod grammer;

use clap::{Arg, App};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

// Programmer defined constants
static PROGRAM_NAME: &'static str = "grammer_sets";

// Derived constants
static VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = App::new(PROGRAM_NAME)
        .version(VERSION)
        .author("Russell W. Bentley <russell_w_bentley@icloud.com>")
        .about("A tool for parsing regular expressions")
        .arg(Arg::with_name("INPUT_FILE")
            .long("input")
            .short("i")
            .value_name("file/path")
            .takes_value(true)
            .required(true))
        .get_matches();

    let input_path = Path::new(args.value_of("INPUT_FILE").unwrap());
    if ! input_path.exists() {
        println!("The passed input file:\n{}\nDoes not exist!", 
            input_path.display()
        );
        exit(1);
    }

    let mut input_file = File::open(&input_path).unwrap();
    let mut input_buffer = Vec::new();
    input_file.read_to_end(&mut input_buffer)
        .expect("Unable to read from file");

    let mut raw_grammer = match parser::parse(&input_buffer) {
        Some(g) => g,
        None => exit(2),
    };

    let grammer = match grammer::check_grammer(&mut raw_grammer) {
        Some(g) => g,
        None => exit(3),
    };
}



