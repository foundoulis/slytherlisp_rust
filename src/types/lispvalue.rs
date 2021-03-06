use std::fmt;
use types::callable::Builtin;
use types::callable::Function;

#[derive(PartialEq, Clone)]
pub enum LispValue {
    String(String),
    Symbol(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    ConsCell(Box<LispValue>, Box<LispValue>),
    Quote(Box<LispValue>),
    Builtin(Box<Builtin>),
    Function(Box<Function>),
    NIL,
}

impl LispValue {
    pub fn from_iterable(list: &Vec<LispValue>) -> LispValue {
        if list.len() == 0 {
            return LispValue::NIL;
        } else if list.len() == 1 {
            return LispValue::ConsCell(Box::new(list[0].clone()), Box::new(LispValue::NIL));
        }
        let (first, new_list) = list.split_first().unwrap();
        LispValue::ConsCell(
            Box::new(first.clone()),
            Box::new(LispValue::from_iterable(&new_list.to_vec())),
        )
    }

    pub fn new_sexpression(left: &LispValue, right: &LispValue) -> LispValue {
        LispValue::ConsCell(Box::new(left.clone()), Box::new(right.clone()))
    }
    pub fn get_values(cell: LispValue) -> Result<(LispValue, LispValue), String> {
        match cell {
            LispValue::ConsCell(left, right) => Ok((*left, *right)),
            _ => Err(String::from("Not a cons cell")),
        }
    }

    pub fn new_quoted(item: LispValue) -> LispValue {
        LispValue::Quote(Box::new(item))
    }
    pub fn get_quoted(item: LispValue) -> Result<LispValue, String> {
        match item {
            LispValue::Quote(item) => Ok(*item),
            _ => Err(String::from("Not a quoted value")),
        }
    }

    pub fn pretty_print(&self) -> String {
        match *self {
            LispValue::String(ref value) => format!("{}", value),
            LispValue::Symbol(ref value) => format!("{}", value),
            LispValue::Integer(value) => value.to_string(),
            LispValue::Float(value) => value.to_string(),
            LispValue::Bool(value) => {
                if value {
                    "#t".to_string()
                } else {
                    "#f".to_string()
                }
            }
            LispValue::ConsCell(ref car, ref cdr) => {
                let (new_car, new_cdr) = self.print_cons(car, cdr);
                format!("({} {})", new_car, new_cdr)
            }
            LispValue::Quote(ref value) => format!("'{}", value),
            LispValue::Function(ref func) => format!("{}", func),
            LispValue::Builtin(ref func) => format!("{:?}", func),
            LispValue::NIL => "NIL".to_string(),
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
            }
            LispValue::NIL => "".to_string(),
            _ => format!(" {}", cdr),
        };
        (car_str, cdr_str)
    }

    pub fn as_bool(&self) -> bool {
        match *self {
            LispValue::NIL => false,
            _ => true,
        }
    }
    pub fn as_list(&self) -> Vec<&LispValue> {
        match *self {
            LispValue::ConsCell(ref left, ref right) => {
                let mut list = right.as_list();
                list.push(left);
                list.reverse();

                list
            }
            LispValue::NIL => vec![],
            _ => vec![],
        }
    }
}

impl fmt::Display for LispValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}
impl fmt::Debug for LispValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}
