
extern crate regex;

use self::regex::Regex;
use std::str::FromStr;
use types::lispvalue::*;

#[derive(Debug, PartialEq)]
pub enum ControlToken {
    LParen,
    RParen,
    Quote,
    Float(f64),
    Int(i64),
    String(String),
    Symbol(String),
    Other // Will be dropped
}
// impl PartialEq for ControlToken {
//     fn eq(&self, &rhs) -> bool {
//         match (self, rhs) {

//         }
//     }
// }

pub fn lex(code: &str) -> Vec<ControlToken> {
    let mut raw_tokens: Vec<ControlToken> = vec![];
    //                           1-( 2-) 3-'    4-float                                        5-int     6-whitespace 7-string    8,9-Comments  10-Symbols            11-Error
    let regex = Regex::new(r###"(\()|(\))|(')|((?:-?[0-9]+[\.][0-9]*)|(?:-?[0-9]*[\.][0-9]+))|(-?[0-9]+)|(\s)|("(?:[^"\\]|\\.)*")|(^#!.*)|(;.*)|([^.\s'"\(\);][^\s'"\(\);]*)|(.*) # 11 Syntax error for anything else."###).unwrap();
    // Using the regex above we are going to build a list of matches with their group, 
    // because of the way the regex rust crate package works, this is very inefficient,
    // however will only be run when an input is handed to the parser.
    for cap in regex.captures_iter(code) {
        // copied from here: https://stackoverflow.com/questions/29126533/how-to-get-the-index-of-matching-group-in-the-regex-crate
        let index = cap.iter().enumerate() 
            .skip(1)                  // skip the first group
            .find(|t| t.1.is_some())  // find the first `Some`: expensive
            .map(|t| t.0)             // extract the index
            .unwrap_or(0);
        raw_tokens.push(match index {
            1 => ControlToken::LParen,
            2 => ControlToken::RParen,
            3 => ControlToken::Quote,
            4 => ControlToken::Float(f64::from_str(&cap[0]).unwrap()),
            5 => ControlToken::Int(i64::from_str(&cap[0]).unwrap()),
            6 => ControlToken::Other,
            7 => ControlToken::String(String::from(&cap[0])),
            8 => ControlToken::Other,
            9 => ControlToken::Other,
            10 => ControlToken::Symbol(parse_strlit(String::from(&cap[0]))),
            _ => panic!("Lexing error, unrecognized string: {}", &cap[0])
        });
    }
    let tokens = raw_tokens.into_iter().filter(|x| *x != ControlToken::Other).collect();
    tokens
}

fn parse_strlit(string: String) -> String {
    // In Rust, this shit is handled automatically
    string
}

fn parse(tokens: Vec<ControlToken>) -> () {

}

fn lisp(code: &str) -> () {
    parse(lex(code))
}

