
pub mod types;

use types::atom::*;
use types::conscell::*;

fn main() {
    let first = ConsCell::new(
        Atom::Operator('+'), 
        Atom::Integer(1000)
    );
    let second = ConsCell::new(
        Atom::Operator('+'), 
        Atom::Integer(100)
    );
    println!("{:?}", first);
}
