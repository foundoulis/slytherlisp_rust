
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum LispValue {
    String(String),
    Symbol(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    ConsCell(Box<LispValue>, Box<LispValue>),
    // Func(Function),
    NIL
}

impl LispValue {
    pub fn from_iterable(list: &Vec<LispValue>) -> LispValue {
        if list.len() == 0 {
            return LispValue::NIL;
        } else if list.len() == 1 {
            return LispValue::ConsCell(Box::new(list[0].clone()), Box::new(LispValue::NIL));
        }
        let (first, new_list) = list.split_first().unwrap();
        LispValue::ConsCell(Box::new(first.clone()), Box::new(LispValue::from_iterable(&new_list.to_vec())))
    }

    pub fn pretty_print(&self) -> String {
        match *self {
            LispValue::String(ref value) => format!("\"{}\"", value),
            LispValue::Symbol(ref value) => format!("{}", value),
            LispValue::Integer(value) => value.to_string(),
            LispValue::Float(value) => value.to_string(),
            LispValue::Bool(value) => if value {
                "#t".to_string()
            } else {
                "#f".to_string()
            },
            LispValue::ConsCell(ref car, ref cdr) => {
                let (new_car, new_cdr) = self.print_cons(car, cdr);
                format!("{} {}", new_car, new_cdr)
            },
            LispValue::NIL => "NIL".to_string()
        }
    }

    fn print_cons(&self, car: &LispValue, cdr: &LispValue) -> (String, String) {
        let car_str = car.pretty_print();
        let cdr_str = match *cdr {
            LispValue::ConsCell(ref new_car, ref new_cdr) => {
                let (temp_car, temp_cdr) = self.print_cons(&*new_car, &*new_cdr);
                if temp_cdr.len() == 0 {
                    format!("{}", temp_car)
                } else {
                    format!("{} {}", temp_car, temp_cdr)
                }
            },
            LispValue::NIL => "".to_string(),
            _ => format!(" {}", cdr)
        };
        (car_str, cdr_str)
    }
    pub fn as_bool(&self) -> bool {
        match *self {
            LispValue::NIL => false,
            _ => true
        }
    }
}

impl fmt::Display for LispValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}
