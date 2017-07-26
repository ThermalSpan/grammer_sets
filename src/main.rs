#![allow(dead_code)]

extern crate clap;
#[macro_use]extern crate nom;

mod args;
mod alphabet;
mod parser;
mod raw_grammer;

use std::process::exit;

fn main () {
    let args = args::parse_args();

    let mut raw_grammer = match parser::parse(&args.input_buffer) {
        Some(g) => g,
        None => exit(2),
    };


}
