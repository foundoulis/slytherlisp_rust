
extern crate regex;

use self::regex::RegexSet;

pub enum ControlToken {
    LParen,
    RParen,
    Quote
}

fn lex(code: String) -> Vec<ControlToken> {
    let reglex = RegexSet::new(&[
        r###"\("###,  // 1
        r###"\)"###,  // 2
        r###"\'"###,  // 3 Control sequence ()'
        r###"(?:-?[0-9]+[\.][0-9]*)|(?:-?[0-9]*[\.][0-9]+)"###,
        r###"-?[0-9]+"###,  // 5 Integer
        r###"\s"###, // 6 Whitespace
        r###""(?:[^"\\]|\\.)*""###,  // 7 String
        r###"^[#!].*"###,  // 8 comments
        r###";.*"###,  // 9 other comments
        r###"[^.\s\'\"\(\);][^\s\'\"\(\);]*"###,  // 10 symbols
        r###".*"### // 11 Syntax error for anything else.
    ]).unwrap();
    reglex.matches(&code).into_iter().collect()
}

fn parse(tokens: Vec<ControlToken>) -> () {

}

fn lisp(code: String) -> () {
    parse(lex(code))
}

