
use std::collections::HashMap;
use std::fmt;
use types::lispvalue::LispValue;


pub trait Callable {
    fn call(&mut self, args: &LispValue) -> LispValue;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Builtin {
    add,
    sub,
    mul,
    div,
    print
}
impl Builtin {
    pub fn new(func: &str) -> Builtin {
        match func {
            "+" => Builtin::add,
            "-" => Builtin::sub,
            "*" => Builtin::mul,
            "/" => Builtin::div,
            "print" => Builtin::print,
            _ => panic!("{} is not builtin.", func),
        }
    }
}
impl Callable for Builtin {
    fn call(&mut self, args: &LispValue) -> LispValue {
        let args_as_list: Vec<&LispValue> = args.as_list();
        match *self {
            Builtin::add => {
                let mut int_sum = 0i64;
                let mut float_sum = 0f64;
                for item in args_as_list {
                    match *item {
                        LispValue::Integer(i) => int_sum += i,
                        LispValue::Float(f) => float_sum += f,
                        _ => panic!("Cannot add type: {}", *item),
                    }
                }
                if int_sum == 0 {
                    LispValue::Float(float_sum)
                } else if float_sum == 0.0 {
                    LispValue::Integer(int_sum) 
                } else {
                    LispValue::Float(float_sum + (int_sum as f64))
                }
            },
            Builtin::sub => {
                match args_as_list.len() {
                    0 => LispValue::Integer(1),
                    1 => {
                        match *args_as_list[0] {
                            LispValue::Integer(i) => LispValue::Integer((-1)*(i)),
                            LispValue::Float(f) => LispValue::Float((-1.0)*(f)),
                            _ => panic!("Cannot negate type: {}", *args_as_list[0]),
                        }
                    },
                    _ => {
                        let mut is_int = match *args_as_list[0] {
                            LispValue::Integer(_) => true,
                            LispValue::Float(_) => false,
                            _ => panic!("Cannot subtract from: {}", *args_as_list[0]),
                        };
                        if is_int {
                            let start = match *args_as_list[0] {
                                LispValue::Integer(i) => i,
                                _ => panic!("impossible"),
                            };
                            let mut total = 0i64;
                            for index in 1..args_as_list.len() {
                                let value = match *args_as_list[index] {
                                    LispValue::Integer(i) => i,
                                    LispValue::Float(f) => f as i64,
                                    _ => panic!("Cannot subtract from: {}", *args_as_list[index])
                                };
                                total += value as i64;
                            }
                            LispValue::Integer(start - total)
                        } else {
                            let start = match *args_as_list[0] {
                                LispValue::Float(f) => f, 
                                _ => panic!("impossible"),
                            };
                            let mut total = 0f64;
                            for index in 1..args_as_list.len() {
                                let value = match *args_as_list[index] {
                                    LispValue::Float(f) => f,
                                    LispValue::Integer(i) => i as f64,
                                    _ => panic!("Cannot subtract from: {}"),
                                };
                                total += value as f64;
                            }
                            LispValue::Float(start - total)
                        }
                    }
                }
            },
            Builtin::mul => {
                let mut int_prod = 1i64;
                let mut float_prod = 1f64;
                for item in args_as_list {
                    match *item {
                        LispValue::Integer(i) => int_prod *= i,
                        LispValue::Float(f) => float_prod *= f,
                        _ => panic!("Cannot multiply type: {}", *item),
                    }
                }
                if int_prod == 1 {
                    LispValue::Integer(int_prod)
                } else if float_prod == 1.0 {
                    LispValue::Float(float_prod)
                } else {
                    LispValue::Float(float_prod * (int_prod as f64))
                }
            },
            Builtin::div => {
                match args_as_list.len() {
                    0 => LispValue::Integer(1),
                    1 => {
                        match *args_as_list[0] {
                            LispValue::Float(f) => LispValue::Float(1f64/f),
                            LispValue::Integer(i) => LispValue::Float(1f64/(i as f64)),
                            _ => panic!("Cannot divide type: {}", *args_as_list[0]),
                        }
                    },
                    _ => {
                        let floats: Vec<f64> = args_as_list.iter().map(|item| {
                            match *item {
                                LispValue::Integer(i) => *i as f64,
                                LispValue::Float(f) => *f,
                                _ => panic!("Cannot divide type: {}", *item),
                            }
                        }).collect();
                        let mut start = floats[0];
                        for index in 1..floats.len() {
                            start /= floats[index];
                        }
                        LispValue::Float(start)
                    }
                }
            },
            Builtin::print => {
                for thing in args_as_list {
                    println!("std out: {}", thing);
                }
                LispValue::NIL
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    params: LispValue,
    body: LispValue,
    environ: HashMap<String, LispValue>
}

impl Function {
    pub fn new(params: LispValue, 
                body: LispValue, 
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

impl Callable for Function {
    fn call(&mut self, args: &LispValue) -> LispValue {
        LispValue::NIL
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_lambda())
    }
}

