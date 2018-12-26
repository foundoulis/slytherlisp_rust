
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

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    iter::FromIterator,
    path::Path,
};

fn _test_type() {
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

fn _test_parse() {
    let code = "(token_2 (\"another string\" 350) (if predicate consequence alternative) (print \"\x41\") ''''1.25 300 \"\x53\x6c\x79\x74\x68\x65\x72\x4C\x69\x73\x70\") ;comment";
    println!("{}", code);
    let mut ast = lisp(code);
    println!("{:?}", &ast[0]);
    lisp_eval(&mut ast[0], &mut LexicalVarStorage::initialize());
}

fn lines_from_file<P>(filename: P) -> String
where P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    String::from_iter(buf.lines()
        .map(|l| l.expect("Could not parse line")))
}

fn run_interpreter() {
    // first we must import file
    let prog = lines_from_file("/home/foundoulis/CSCI400/slytherlisp_rust/test.scm");

    // Then pass to the interpreter
    let mut interpreter = Interpreter::interpreter();
    interpreter.exec(prog);
}

fn main() {
    run_interpreter();
}
