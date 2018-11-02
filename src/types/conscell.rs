
use std::fmt;

use types::atom::Atom;

pub struct ConsCell {
    pub car: Atom,
    pub cdr: Atom
}

impl ConsCell {
    pub fn new(car: Atom, cdr: Atom) -> ConsCell {
        ConsCell{
            car: car,
            cdr: cdr,
        }
    } 
}
impl PartialEq for ConsCell {
    fn eq(&self, other: &ConsCell) -> bool {
        self.car == other.car && self.cdr == other.cdr
    }
}
impl fmt::Debug for ConsCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?} {:?})", self.car, self.cdr)
    }
}