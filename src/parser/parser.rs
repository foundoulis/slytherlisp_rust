
extern crate regex;

use regex::Regex;

pub enum ControlToken {
    LParen,
    RParen,
    Quote
}

fn lex(code: String) -> Vec<ControlToken> {
    let reglex = Regex::new();
    vec![]
}

fn parse(tokens: Vec<ControlToken>) -> () {

}

fn lisp(code: String) -> () {
    parse(lex(code))
}

