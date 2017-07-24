#![allow(dead_code)]

extern crate clap;
extern crate nom;

mod args;
mod alphabet;

fn main () {
    let args = args::parse_args();
}
