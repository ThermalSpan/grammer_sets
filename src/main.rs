extern crate clap;
#[macro_use] extern crate nom; 

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

    let g = parse_grammer(&input_buffer).unwrap().1;

    println!("{:?}", g);
}

#[derive(Debug)]
enum SymbolClass {
    NonTerminal,
    Terminal,
}

#[derive(Debug)]
struct Symbol {
    name: String,
    class: SymbolClass,
}

#[derive(Debug)]
struct RawRule {
    head:String,
    alternate: Vec<String>
}

#[derive(Debug)]
struct RawGrammer {
    start: String,
    terminals:  Vec<String>,
    non_terminals: Vec<String>,
    rules: Vec<RawRule>
}

named!(parse_grammer <RawGrammer>,
    dbg_dmp!(ws!(do_parse!(
        tag!(":Start:") >>
        start: call!(parse_symbol_name) >>
        tag!(":Terminals:") >>
        terminals: many0!(call!(parse_symbol_name)) >> 
        tag!(":NonTerminals:") >>
        non_terminals: many0!(call!(parse_symbol_name)) >>
        tag!(":Rules:") >>
        rules: call!(parse_rules) >>
        (RawGrammer {
            start: start,
            terminals: terminals,
            non_terminals: non_terminals,
            rules: rules
        })
    )))
);

named!(parse_rules <Vec<RawRule>>,
    dbg_dmp!(many1!(
        call!(parse_rule)
    ))
);

named!(parse_rule <RawRule>,
    dbg_dmp!(ws!(do_parse!(
        head: call!(parse_symbol_name) >>
        tag!("->") >>
        alternate: many1!(
            call!(parse_symbol_name)
        ) >>
        tag!(".") >>
        (RawRule {
            head: head,
            alternate: alternate
        })
    )))
);

named!(parse_symbol_name <String>,
    do_parse!(
        name: call!(nom::alphanumeric) >>
        (String::from(String::from_utf8_lossy(name))
        )
    )
);
