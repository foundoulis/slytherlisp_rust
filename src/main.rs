
pub mod types;
pub mod parser;

use types::lispvalue::*;
use types::conslist::*;

fn main() {
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
        LispValue::String("words".to_string())]).as_list());
}
