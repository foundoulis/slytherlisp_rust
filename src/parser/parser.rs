
extern crate regex;

use regex::RegexSet;

pub enum ControlToken {
    LParen,
    RParen,
    Quote
}

fn lex(code: String) -> Vec<ControlToken> {
    let reglex = RegexSet::new(&[
        r"\(", 
        r"\)",
        r"(?:-?[0-9]+[\.][0-9]*)|(?:-?[0-9]*[\.][0-9]+)",
        r"-?[0-9]+",
        r"\s",
        r""(?:[^"\\]|\\.)*"",
        r"^[#!].*",
        r";.*",
        r"[^.\s\'\"\(\);][^\s\'\"\(\);]*",
        r".*"
    ]);
    reglex.matches(code).into_iter().collect()
}

fn parse(tokens: Vec<ControlToken>) -> () {

}

fn lisp(code: String) -> () {
    parse(lex(code))
}

