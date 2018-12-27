
pub mod evaluator;
pub mod interpreter;
pub mod parser;
pub mod types;

use interpreter::*;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    iter::FromIterator,
    path::Path,
};

fn lines_from_file<P>(filename: P) -> String
where P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    String::from_iter(buf.lines()
        .map(|l| l.expect("Could not parse line")))
}

fn run_file() {
    // first we must import file
    let prog = lines_from_file("/home/foundoulis/CSCI400/slytherlisp_rust/test.scm");

    // Then pass to the interpreter
    let mut interpreter = Interpreter::interpreter();
    interpreter.exec(prog);
}

fn run_repl() {
    
}

fn main() {
    run_file();
}
