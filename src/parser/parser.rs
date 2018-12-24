
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
    Integer(i64),
    String(String),
    Symbol(String),
    Other // Comments and whitespace to be removed,
}

#[derive(Clone, Debug, PartialEq)]
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
    String::from(string)
}

fn parse(tokens: Vec<ControlToken>) -> Result<Vec<ParseToken>, String> {
    let parse_tokens: Vec<ParseToken> = tokens.iter().map(|t| { match t {
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
    let mut return_value: Vec<ParseToken> = Vec::new();
    'main: for elem in parse_tokens {
        let mut was_lparen = false;
        match elem {
            ParseToken::RParen => {
                let mut start = ParseToken::Value(LispValue::NIL);
                'reverse: while stack.len() > 0 {
                    let check = stack[stack.len() - 1].clone();
                    match check {
                        ParseToken::LParen => {
                            was_lparen = true;
                            stack.pop();
                            stack.push(start);
                            break 'reverse;
                        },
                        ParseToken::SingleQuote => {
                            return Err(String::from("Invalid Quotation"));
                        },
                        ParseToken::Value(v) => {
                            let right: LispValue = match start {
                                ParseToken::Value(ref v) => v.clone(),
                                _ => panic!("impossible")
                            };
                            start = ParseToken::Value(LispValue::new_sexpression(&v, &right));
                            stack.pop();
                        },
                        ParseToken::RParen => {
                            // Do nothing
                        }
                    }
                }
                if !was_lparen {
                    return Err(String::from("Too many closing parens"));
                }
            },
            _ => {
                stack.push(elem.clone());
                match elem {
                    ParseToken::Value(_) => {
                        // Do nothing
                    },
                    _ => {
                        continue 'main;
                    }
                }
            }
        }
        while stack.len() > 1 && stack[stack.len() - 2] == ParseToken::SingleQuote {
            let itm = stack.pop(); // Remove last item
            stack.pop(); // Remove quote
            stack.push(match itm.unwrap() {
                ParseToken::Value(v) => ParseToken::Value(LispValue::new_quoted(v)),
                _ => panic!("Oh no.")
            });
        }
        if stack.len() == 1 {
            return_value.push(stack.pop().unwrap());
        }
    }
    if stack.len() > 0 {
        return Err(String::from("incomplete parse"));
    }

    Ok(return_value)
}

pub fn lisp(code: &str) -> Vec<LispValue> {
    let wrapped_ast: Vec<ParseToken> = parse(lex(code)).unwrap_or_else(|text| panic!("Parse Error: {}", text));
    return wrapped_ast.into_iter().map(|value| match value {
        ParseToken::Value(s) => s,
        _ => panic!("What?"),
    }).collect::<Vec<LispValue>>();
}

