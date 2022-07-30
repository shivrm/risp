use super::{Type, WrappedType};
use crate::risp::runtime::{Interpreter, RuntimeError};
use crate::risp::{shared::Op, AstNode};

pub type Str = String;
pub type List = Vec<WrappedType>;
pub type RustFn = fn(List) -> Result<List, RuntimeError>;
pub type RustMacro = fn(&mut Interpreter, &[AstNode]) -> Result<WrappedType, RuntimeError>;
pub struct Null;

impl Type for Str {
    fn display(&self) -> String {
        self.clone()
    }

    fn repr(&self) -> String {
        format!("{self:?}")
    }

    fn add(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Str(s) => (self.clone() + &s).into(),
            _ => return None,
        };
        Some(res)
    }

    fn mul(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => self.repeat(*n as usize).into(),
            _ => return None,
        };
        Some(res)
    }

    fn rmul(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => self.repeat(*n as usize).into(),
            _ => return None,
        };
        Some(res)
    }

    fn eq(&self, other: &WrappedType) -> Option<bool> {
        let res = match other {
            WrappedType::Str(s) => (self == s),
            _ => return None,
        };
        Some(res)
    }
}

impl Type for List {
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

    fn add(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::List(el) => self
                .iter()
                .cloned()
                .chain(el.iter().cloned())
                .collect::<List>()
                .into(),
            _ => return None,
        };
        Some(res)
    }

    fn mul(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => self
                .iter()
                .cloned()
                .cycle()
                .take(self.len() * *n as usize)
                .collect::<List>()
                .into(),
            _ => return None,
        };
        Some(res)
    }

    fn rmul(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => self
                .iter()
                .cloned()
                .cycle()
                .take(self.len() * *n as usize)
                .collect::<List>()
                .into(),
            _ => return None,
        };
        Some(res)
    }

    fn eq(&self, other: &WrappedType) -> Option<bool> {
        match other {
            WrappedType::List(l) => Some(
                (self.len() == l.len())
                    && self.iter().zip(l).all(|(a, b)| a.eq(b).unwrap_or(false)),
            ),
            _ => None,
        }
    }
}

impl Type for RustFn {
    fn display(&self) -> String {
        "<Rust Function>".to_owned()
    }

    fn repr(&self) -> String {
        self.display()
    }
}

impl Type for RustMacro {
    fn display(&self) -> String {
        "<Rust Macro>".to_owned()
    }

    fn repr(&self) -> String {
        self.display()
    }
}

impl Type for Op {
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
}

impl Type for Null {
    fn display(&self) -> String {
        "Null".to_owned()
    }

    fn repr(&self) -> String {
        String::new()
    }
}
