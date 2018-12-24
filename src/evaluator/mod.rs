
use types::lispvalue::*;
use types::lexicalvarstorage::*;

pub fn lisp_eval(expr: &LispValue, stg: &mut LexicalVarStorage) -> LispValue {
    println!("In evaluation: {}", expr);
    match *expr {
        LispValue::NIL => return LispValue::NIL,
        LispValue::Quote(ref elem) => {
            match **elem {
                LispValue::ConsCell(ref car, ref cdr) => {
                    println!("{} {}", car, cdr);
                    *cdr.clone()
                },
                _ => *elem.clone(),
            }
        },
        LispValue::Symbol(ref symb) => {
            stg.get(symb.to_string()).unwrap_or_else(|err| {
                panic!(err);
            })
        },
        LispValue::ConsCell(ref car, ref cdr) => {
            let _output = lisp_eval(car, stg);
            
            LispValue::NIL
        },
        _ => expr.clone()
    }
}
