use nom::{alphanumeric, ErrorKind, IResult};
use nom::verbose_errors::Err;
use raw_grammer::*;

const START_ERR: u32 = 1;
const TERMINALS_ERR: u32 = 2;
const NONTERMINALS_ERR: u32 = 3;
const NAME_ERR: u32 = 5;
const NAMES_ERR: u32 = 6;
const RULES_ERR: u32 = 7;
const RULES_LIST_ERR: u32 = 8;
const RULE_HEAD_ERR: u32 = 9;
const RULE_ARROW_ERR: u32 = 10;
const RULE_TERM_ERR: u32 = 11;

fn error_code_to_str(code: u32) -> &'static str {
    match code {
        START_ERR => "Expected ':Start:'",
        TERMINALS_ERR => "Expected ':Terminals:'",
        NONTERMINALS_ERR => "Expected ':NonTerminals:'",
        NAME_ERR => "Expected a single alphanumeric name",
        NAMES_ERR => "Expected a whitespace seperated list of alphanumeric names",
        RULES_ERR => "Expected ':Rules:'",
        RULES_LIST_ERR => "Expected a whitespace seperated list of rules",
        RULE_HEAD_ERR => "Expected alphanumeric name for head of rule",
        RULE_ARROW_ERR => "Expected '->'",
        RULE_TERM_ERR => "Expected '.'",
        _ => "Unknown Error code?"
    }
}

fn error_kind_to_str(err: &ErrorKind) -> String {
    match *err {
        ErrorKind::Custom(code) => String::from(error_code_to_str(code)),
        _ => format!("{:?}", err),
    }
}

fn print_error(err: &Err<&[u8]>) {
    match *err {
        Err::Code(ref code) => println!("ERROR: There was an error {}", error_kind_to_str(code)),
        Err::Node(ref code, ref errs) => {
            println!(
                "ERROR: There was a node error: {}\n It contained {} other errors", 
                error_kind_to_str(code),
                errs.len()
            );
            let _: Vec<()> = errs.iter().map(|e| print_error(e)).collect();
        },
        Err::Position(ref code, ref pos) => {
            println!("ERROR: There was an error at {}:\n{}", String::from_utf8_lossy(pos), error_kind_to_str(code));
        },
        Err::NodePosition(ref code, ref pos, ref errs) => {
            println!(
                "ERROR: There was an node error at {}:\n{}\nThere were {} errors contained",
                String::from_utf8_lossy(pos),
                error_kind_to_str(code),
                errs.len()
            );
        }
    }
}

pub fn parse(input: &[u8]) -> Option<RawGrammer> {
    let result = parse_grammer(input);

    match result {
        IResult::Done(leftover_input, grammer) => {
            if leftover_input.len() > 0 {
                println!(
                    "There was some leftover input?:\n{}\n==============\n",
                    String::from_utf8_lossy(leftover_input)
                );
                None
            } else {
                Some(grammer)
            }
        },
        IResult::Error(err) => {
            print_error(&err);
            None
        },
        IResult::Incomplete(needed) => {
            println!("ERROR: nom expected some more bytes, how many? {:?}", needed);
            None
        }
    }
}

named!(parse_grammer <RawGrammer>,
    ws!(do_parse!(
        add_return_error!(
            ErrorKind::Custom(START_ERR), 
            tag!(":Start:") 
        ) >>
        start: add_return_error!(
            ErrorKind::Custom(NAME_ERR),
            call!(parse_symbol_name) 
        ) >>
        add_return_error!(
            ErrorKind::Custom(TERMINALS_ERR),
            tag!(":Terminals:")
        ) >>
        terminals: add_return_error!(
            ErrorKind::Custom(NAMES_ERR),
            call!(parse_name_vec)
        ) >> 
        add_return_error!(
            ErrorKind::Custom(NONTERMINALS_ERR),
            tag!(":NonTerminals:")
        ) >>
        non_terminals: add_return_error!(
            ErrorKind::Custom(NAMES_ERR),
            call!(parse_name_vec)
        ) >>
        add_return_error!(
            ErrorKind::Custom(RULES_ERR),
            tag!(":Rules:")
        ) >>
        rules: add_return_error!(
            ErrorKind::Custom(RULES_LIST_ERR),
            call!(parse_rules)
        ) >>
        eof!() >>
        (RawGrammer {
            start: start,
            terminals: terminals,
            non_terminals: non_terminals,
            rules: rules
        })
    ))
);

named!(parse_name_vec <Vec<String>>, 
    ws!(many1!(call!(parse_symbol_name)))
);

named!(parse_rules <Vec<RawRule>>,
    many1!(
        call!(parse_rule)
    )
);

named!(parse_rule <RawRule>,
    ws!(do_parse!(
        head: add_return_error!(
            ErrorKind::Custom(RULE_HEAD_ERR),
            call!(parse_symbol_name) 
        ) >>
        add_return_error!(
            ErrorKind::Custom(RULE_ARROW_ERR),
            tag!("->") 
        ) >>
        alternate: add_return_error!(
            ErrorKind::Custom(NAMES_ERR),
            call!(parse_name_vec)
        ) >>
        add_return_error!(
            ErrorKind::Custom(RULE_TERM_ERR),
            tag!(".")
        ) >>
        (RawRule {
            head: head,
            alternate: alternate
        })
    ))
);

named!(parse_symbol_name <String>,
    do_parse!(
        name: call!(alphanumeric) >>
        (String::from(String::from_utf8_lossy(name))
        )
    )
);
