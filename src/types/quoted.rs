
use std::fmt;
use types::lispvalue::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Quoted(Box<LispValue>);

impl Quoted {
    pub fn new(elem: LispValue) -> Quoted {
        Quoted(Box::new(elem))
    }
}
impl fmt::Display for Quoted {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}", self.0)
    }
}
