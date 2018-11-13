
extern crate regex;

use self::regex::RegexSet;
use types::lispvalue::*;

#[derive(Debug)]
pub enum ControlToken {
    LParen,
    RParen,
    Quote,
    Other
}

pub fn lex(code: &str) -> Vec<ControlToken> {
    let mut tokens: Vec<ControlToken> = vec![];
    let reglex = RegexSet::new(&[
        r###"\("###,  // 1
        r###"\)"###,  // 2
        r###"'"###,  // 3 Control sequence ()'
        r###"(?:-?[0-9]+[\.][0-9]*)|(?:-?[0-9]*[\.][0-9]+)"###,
        r###"-?[0-9]+"###,  // 5 Integer
        r###"\s"###, // 6 Whitespace
        r###""(?:[^"\\]|\\.)*""###,  // 7 String
        r###"^[#!].*"###,  // 8 comments
        r###";.*"###,  // 9 other comments
        r###"[^.\s'"\(\);][^\s'"\(\);]*"###,  // 10 symbols
        r###".*"### // 11 Syntax error for anything else.
    ]).unwrap();
    for tok in reglex.captures_iter(code) {
        println!("{}", tok);
        tokens.push(match tok {
            0 => ControlToken::LParen,
            1 => ControlToken::RParen,
            _ => ControlToken::Other
        });
    }
    println!("{:?}", tokens);
    tokens
}

fn parse_strlit(string: &str) -> LispValue {
    let reglex = RegexSet::new(&[
        r###"\\0[0-7]{2}"###, // octal
        r###"\\x[a-fA-F0-9]{2}"###, // hex
        r###"\\0"###,
        r###"\\a"###,
        r###"\\b"###,
        r###"\\e"###,
        r###"\\f"###,
        r###"\\n"###,
        r###"\\r"###,
        r###"\\t"###,
        r###"\\v"###,
        r###"\\""###,
        r###"\\[^a-zA-Z]"###, // escaped character
        r###"""###,
        r###"."###,
    ]).unwrap();
    LispValue::NIL
}

fn parse(tokens: Vec<ControlToken>) -> () {

}

fn lisp(code: &str) -> () {
    parse(lex(code))
}

