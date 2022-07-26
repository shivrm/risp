use super::{Type, RispType};
use crate::risp::{AstNode, shared::Op};
use crate::risp::runtime::{Interpreter, RuntimeError};

pub type Str = String;
pub type List = Vec<Type>;
pub type RustFn = fn(List) -> Result<List, RuntimeError>;
pub type RustMacro = fn(&mut Interpreter, Vec<AstNode>) -> Result<Type, RuntimeError>;
pub struct Null;


impl RispType for Str {
    fn display(&self) -> String {
        self.clone()
    }

    fn repr(&self) -> String {
        format!("{self:?}")
    }

    fn type_name(&self) -> String {
        return "str".into()
    }

    fn add(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Str(s) => (self.clone() + &s).into(),
            _ => return None,
        };
        Some(res)
    }

    fn mul(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => self.repeat(*n as usize).into(),
            _ => return None,
        };
        Some(res)
    }

    fn rmul(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => self.repeat(*n as usize).into(),
            _ => return None,
        };
        Some(res)
    }

    fn equal(&self, other: &Type) -> Option<bool> {
        let res = match other {
            Type::Str(s) => (self == s),
            _ => return None,
        };
        Some(res)
    }
}

impl RispType for List {
    fn repr(&self) -> String {
        let mut iter = self.iter();

        let mut result = match iter.next() {
            Some(el) => {
                let r = el.repr();
                format!("[{r}")
            }
            None => String::from("["),
        };

        for el in iter {
            result += &format!(", {}", el.repr())
        }

        result.push(']');

        result
    }

    fn display(&self) -> String {
        let mut iter = self.iter();

        let mut result = match iter.next() {
            Some(el) => format!("[{}", el.display()),
            None => String::from("["),
        };

        for el in iter {
            result += &format!(", {}", el.display())
        }

        result.push(']');

        result
    }

    fn type_name(&self) -> String {
        return "list".into()
    }

    fn add(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::List(el) => self.iter().cloned().chain(el.iter().cloned()).collect::<List>().into(),
            _ => return None,
        };
        Some(res)
    }

    fn mul(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => self.iter()
                    .cloned()
                    .cycle()
                    .take(self.len() * *n as usize)
                    .collect::<List>()
                    .into(),
            _ => return None,
        };
        Some(res)
    }

    fn rmul(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => self.iter()
                    .cloned()
                    .cycle()
                    .take(self.len() * *n as usize)
                    .collect::<List>()
                    .into(),
            _ => return None,
        };
        Some(res)
    }

    fn equal(&self, other: &Type) -> Option<bool> {
        match other {
            Type::List(l) => Some((self.len() == l.len()) && self.iter().zip(l).all(|(a,b)| a.equal(b).unwrap_or(false))),
            _ => None,
        }
    }
}

impl RispType for RustFn {
    fn display(&self) -> String {
        "<Rust Function>".to_owned()
    }

    fn repr(&self) -> String {
        self.display()
    }

    fn type_name(&self) -> String {
        return "rustfn".into()
    }
}


impl RispType for RustMacro {
    fn display(&self) -> String {
        "<Rust Macro>".to_owned()
    }

    fn repr(&self) -> String {
        self.display()
    }

    fn type_name(&self) -> String {
        return "rustmacro".into()
    }
}

impl RispType for Op {
    fn display(&self) -> String {
        let value = match self {
            Op::Plus => "+",
            Op::Minus => "-",
            Op::Star => "*",
            Op::Slash => "/",
            Op::Equal => "=",
            Op::Greater => ">",
            Op::Less => "<",
        };
        value.to_owned()
    }

    fn repr(&self) -> String {
        self.display()
    }

    fn type_name(&self) -> String {
        return "operator".into()
    }
}

impl RispType for Null {
    fn display(&self) -> String {
        "Null".to_owned()
    }

    fn repr(&self) -> String {
        String::new()
    }

    fn type_name(&self) -> String {
        return "null".into()
    }
}
