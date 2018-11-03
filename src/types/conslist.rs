
use std::fmt;

use types::lispvalue::LispValue;

pub struct ConsList(LispValue);

impl ConsList {
    pub fn new(list: &Vec<LispValue>) -> ConsList {
        ConsList(LispValue::from_iterable(list))
    }
}
impl fmt::Display for ConsList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
