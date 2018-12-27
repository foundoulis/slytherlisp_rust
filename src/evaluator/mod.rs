
use types::callable::Callable;
use types::lispvalue::*;
use types::lexicalvarstorage::*;

pub fn lisp_eval(expr: LispValue, stg: &mut LexicalVarStorage) -> LispValue {
    println!("In evaluation: {}", expr);
    match expr {
        LispValue::NIL => LispValue::NIL,
        LispValue::Quote(ref elem) => {
            match **elem {
                LispValue::ConsCell(ref car, ref cdr) => {
                    let mut the_list: Vec<LispValue> = vec![(**car).clone()];
                    let new_thing = LispValue::new_quoted((**car).clone());
                    the_list.push(lisp_eval(new_thing, stg));
                    for x in cdr.as_list().iter() {
                        let thing = LispValue::new_quoted((**x).clone());
                        the_list.push(lisp_eval(thing, stg));
                    }
                    LispValue::from_iterable(&the_list)
                },
                _ => *elem.clone(),
            }
        },
        LispValue::Symbol(ref symb) => {
            stg.get(symb.to_string()).unwrap_or_else(|err| {
                panic!(err);
            })
        },
        LispValue::ConsCell(car, cdr) => {
            match lisp_eval(*car, stg) {
                LispValue::Builtin(ref mut b) => {
                    (**b).call(&cdr)
                },
                _ => LispValue::NIL
            }
        },
        _ => expr.clone()
    }
}
