
use std::fmt;
use std::ops::Index;

use types::lispvalue::LispValue;

#[derive(Clone, PartialEq)]
pub struct ConsList(LispValue, usize);

impl ConsList {
    pub fn new(list: &Vec<LispValue>) -> ConsList {
        ConsList(LispValue::from_iterable(list), list.len())
    }
    pub fn len(&self) -> usize {
        self.1
    }
    pub fn as_list(&self) -> Vec<&LispValue> {
        let mut new_vec: Vec<&LispValue> = vec![];
        let mut curr = &self.0;
        while *curr != LispValue::NIL {
            new_vec.push(curr);
            curr = match curr {
                LispValue::ConsCell(ref car, ref _cdr) => car,
                _ => break,
            };
        }
        new_vec.pop(); // Needs to happen, don't know why.
        new_vec
    }
    pub fn conains(&self, p: LispValue) -> bool {
        for elem in self.as_list() {
            if *elem == p {
                return true;
            }
        }
        false
    }
}
impl Index<usize> for ConsList {
    type Output = LispValue;
    fn index(&self, index: usize) -> &LispValue {
        if index > self.1 {
            return &LispValue::NIL;
        }
        let mut curr = &self.0;
        let mut start: usize = 0;
        while start < index {
            curr = match curr {
                LispValue::ConsCell(ref _car, ref cdr) => cdr,
                _ => break
            };
            start = start + 1;
        }
        match curr {
            LispValue::ConsCell(ref car, ref _cdr) => &car,
            _ => &LispValue::NIL
        }
    }
}
impl fmt::Display for ConsList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(list {})", self.0)
    }
}
impl fmt::Debug for ConsList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(list {})", self.0)
    }
}
