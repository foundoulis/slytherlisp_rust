
use std::collections::HashMap;
use std::fmt;
use types::lispvalue::LispValue;
use types::conslist::ConsList;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    params: ConsList,
    body: ConsList,
    environ: HashMap<String, LispValue>
}

impl Function {
    pub fn new(params: ConsList, 
                body: ConsList, 
                environ: HashMap<String, LispValue>) -> Function {
        Function {
            params: params,
            body: body,
            environ: environ
        }
    }
    pub fn as_lambda(&self) -> String {
        format!("(lambda ({}) {})", self.params, self.body)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_lambda())
    }
}

