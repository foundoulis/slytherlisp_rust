
use std::fmt;

#[derive(PartialEq)]
pub enum Atom {
    Operator(char),
    Symbol(char),

    Integer(i64),
    Float(f64),

    Bool(bool),
}
impl Atom {
    pub fn get(&self) -> Option {
        match self {
            Operator(value) => Some(value),
            _ => None
        }
    }
}

impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
