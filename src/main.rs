
pub mod evaluator;
pub mod interpreter;
pub mod parser;
pub mod types;

use evaluator::lisp_eval;
use interpreter::*;
use parser::parser::*;
use types::lexicalvarstorage::*;
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
    let code = "(token_2 (\"another string\" 350) (if predicate consequence alternative) (print \"\x41\") ''''1.25 300 \"\x53\x6c\x79\x74\x68\x65\x72\x4C\x69\x73\x70\") ;comment";
    println!("{}", code);
    let mut ast = lisp(code);
    println!("{:?}", &ast[0]);
    lisp_eval(&mut ast[0], &mut LexicalVarStorage::initialize());
}

fn run_evaluator() {
    let mut interpreter = Interpreter::interpreter();
    
}

fn main() {
    run_evaluator();
    test_parse();
}
