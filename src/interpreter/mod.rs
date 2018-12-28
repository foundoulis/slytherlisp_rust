use evaluator::*;
use parser::parser::lisp;
use types::lexicalvarstorage::*;
use types::lispvalue::*;

#[derive(Debug)]
pub struct Interpreter {
    stg: LexicalVarStorage,
}
impl Interpreter {
    pub fn interpreter() -> Interpreter {
        Interpreter {
            stg: LexicalVarStorage::initialize(),
        }
    }
    pub fn eval(&mut self, expr: LispValue) -> LispValue {
        lisp_eval(expr, &mut self.stg)
    }
    pub fn exec(&mut self, code: String) -> LispValue {
        let mut r = LispValue::NIL;
        for expr in lisp(&code) {
            r = self.eval(expr);
        }
        r
    }
}
