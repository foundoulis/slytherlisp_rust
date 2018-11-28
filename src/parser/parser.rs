
extern crate regex;

use self::regex::Regex;
use std::str::FromStr;
use types::lispvalue::*;
use types::conslist::*;
use types::quoted::*;

#[derive(Debug, PartialEq)]
pub enum ControlToken {
    LParen,
    RParen,
    Quote,
    Float(f64),
    Integer(i64),
    String(String),
    Symbol(String),
    Other // Comments and whitespace to be removed,
}

#[derive(Debug)]
pub enum ParseToken {
    LParen, 
    RParen,
    SingleQuote,
    Value(LispValue),
}

fn lex(code: &str) -> Vec<ControlToken> {
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
            5 => ControlToken::Integer(i64::from_str(&cap[0]).unwrap()),
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
    // # tokens is our generator
    // stack = []
    // token_copy = tokens
    // for elem in token_copy:
    //     """
    //     se = Nil
    //     while type(stack[-1] is not LParen):
    //         itm = stack.pop()
    //         se = SE(itm, se)
    //     """
    //     was_lparen = False

    //     if elem is RParen():
    //         start = NIL
    //         for x in reversed(stack):
    //             check = x
    //             if check is LParen():
    //                 was_lparen = True
    //                 stack.pop()
    //                 stack.append(start)
    //                 break
    //             elif (isinstance(check, SExpression)
    //                     or isinstance(check, Symbol)
    //                     or isinstance(check, int)
    //                     or isinstance(check, float)
    //                     or isinstance(check, String)
    //                     or isinstance(check, Quoted)
    //                     or check is NIL):
    //                 start = SExpression(check, start)
    //                 stack.pop()
    //             elif isinstance(check, Quote):
    //                 raise SyntaxError("invalid quotation")
    //         if not was_lparen:
    //             raise SyntaxError("too many closing parens")

    //     else:
    //         stack.append(elem)
    //         if isinstance(elem, ControlToken):
    //             continue
    //     while len(stack) > 1 and isinstance(stack[-2], Quote):
    //             itm = stack.pop()
    //             stack[-1] = Quoted(itm)
    //     if len(stack) == 1:
    //         yield stack.pop()

    // if len(stack) > 0:
    //     raise SyntaxError('incomplete parse')
fn parse(tokens: Vec<ControlToken>) -> Result<Vec<ParseToken>, String> {
    let mut parse_tokens: Vec<ParseToken> = tokens.iter().map(|t| { match t {
        ControlToken::LParen => ParseToken::LParen,
        ControlToken::RParen => ParseToken::RParen,
        ControlToken::Float(f) => ParseToken::Value(LispValue::Float(*f)),
        ControlToken::Integer(i) => ParseToken::Value(LispValue::Integer(*i)),
        ControlToken::Quote => ParseToken::SingleQuote,
        ControlToken::String(s) => ParseToken::Value(LispValue::String(s.to_string())),
        ControlToken::Symbol(s) => ParseToken::Value(LispValue::Symbol(s.to_string())),
        _ => panic!("parsing error")
    }}).collect();

    let mut stack: Vec<ParseToken> = Vec::new();
    for elem in parse_tokens {
        let mut was_lparen = false;

        if elem == ParseToken::RParen {
            let mut start = LispValue::NIL;
        } else {
            stack.push(elem);
            if elem != ParseToken::Value() {
                continue;
            }
        }

        while stack.len() > 1 && stack[stack.len()-2] == ParseToken::SingleQuote {
            let itm = stack.pop().unwrap();
            stack[stack.len()-1] = ParseToken::Value(Quoted::new(itm));
        }
    }

    Ok(parse_tokens)
}

pub fn lisp(code: &str) -> () {
    println!("{:?}", parse(lex(code)).unwrap());
}

