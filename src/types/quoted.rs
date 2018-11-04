
use std::fmt;

#[derive(PartialEq)]
pub struct Quoted(LispValue);

impl Quoted {
    pub fn new(elem: LispValue) -> Quoted {
        Quoted(elem)
    }
}
impl fmt::Display for Quoted {
    fn fmt(&self, f: &mut fmt::Formatter) fmt::Result {
        write!(f, "'{}", self.0)
    }
}
