
pub mod parser;
pub mod types;

use parser::parser::*;
use types::lispvalue::*;
use types::conslist::*;

fn test_type() {
    let first = LispValue::ConsCell(
        Box::new(LispValue::Symbol("+".to_string())), 
        Box::new(LispValue::Integer(1000))
    );
    let second = LispValue::ConsCell(
        Box::new(LispValue::Symbol("+".to_string())), 
        Box::new(LispValue::Integer(1000))
    );
    println!("{}", first);
    println!("{}", first == second);
    println!("{:?}", ConsList::new(&vec![
        LispValue::Bool(true), 
        LispValue::Float(3.0f64),
        LispValue::Symbol("Hello World.".to_string()),
        LispValue::String("words".to_string())
    ]).as_list());
}

fn test_parse() {
    let code = "(print (\"\x41\") '1.25 300 \"\x53\x6c\x79\x74\x68\x65\x72\x4C\x69\x73\x70\") ;comment";
    println!("{}", code);
    println!("{:?}", lisp(code));
}

fn main() {
    test_parse();
}
