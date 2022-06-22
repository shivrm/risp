use crate::risp::{ Interpreter, AstNode, Op, RuntimeError };

pub type Int = i32;
pub type Float = f64;
pub type Str = String;
pub type List = Vec<Type>;
pub type RustFn = fn(List) -> Result<List, RuntimeError>;
pub type RustMacro = fn(&mut Interpreter, Vec<AstNode>) -> Result<Type, RuntimeError>;
pub struct Null;

macro_rules! delegate {
    ($obj:ident, $name:ident, $( $x:expr ),*) => {
        match $obj {
            Type::Int(n)    => n.$name($($x)*),
            Type::Float(f)  => f.$name($($x)*),
            Type::Str(s)    => s.$name($($x)*),
            Type::List(l)   => l.$name($($x)*),
            Type::RustFn(f) => f.$name($($x)*),
            Type::RustMacro(f) => f.$name($($x)*),
            Type::Operator(op) => op.$name($($x)*),
            Type::Null      => Null.$name($($x)*),
        }
    };
}

#[derive(Clone)]
pub enum Type {
    Int(Int),
    Float(Float),
    Str(Str),
    List(List),
    RustFn(RustFn),
    RustMacro(RustMacro),
    Operator(Op),
    Null,
}

pub trait RispType {
    fn repr(&self) -> String;
    fn display(&self) -> String;
    fn type_name(&self) -> String;
    
    fn add(&self, _other: &Type) -> Option<Type> {
        None
    }
    fn sub(&self, _other: &Type) -> Option<Type> {
        None
    }
    fn mul(&self, _other: &Type) -> Option<Type> {
        None
    }
    fn div(&self, _other: &Type) -> Option<Type> {
        None
    }
    fn radd(&self, _other: &Type) -> Option<Type> {
        None
    }
    fn rsub(&self, _other: &Type) -> Option<Type> {
        None
    }
    fn rmul(&self, _other: &Type) -> Option<Type> {
        None
    }
    fn rdiv(&self, _other: &Type) -> Option<Type> {
        None
    }
}

// Function definitions could be done by a macro 🤔
impl RispType for Type {
    fn repr(&self) -> String {
        delegate!(self, repr,)
    }
    
    fn display(&self) -> String {
        delegate!(self, display,)
    }
    
    fn type_name(&self) -> String {
        delegate!(self, type_name,)
    }

    fn add(&self, other: &Type) -> Option<Type> {
        delegate!(self, add, other)
    }

    fn sub(&self, other: &Type) -> Option<Type> {
        delegate!(self, sub, other)
    }

    fn mul(&self, other: &Type) -> Option<Type> {
        delegate!(self, mul, other)
    }

    fn div(&self, other: &Type) -> Option<Type> {
        delegate!(self, div, other)
    }

    fn radd(&self, other: &Type) -> Option<Type> {
        delegate!(self, add, other)
    }

    fn rsub(&self, other: &Type) -> Option<Type> {
        delegate!(self, sub, other)
    }

    fn rmul(&self, other: &Type) -> Option<Type> {
        delegate!(self, mul, other)
    }

    fn rdiv(&self, other: &Type) -> Option<Type> {
        delegate!(self, div, other)
    }
}

impl RispType for Int {
    fn display(&self) -> String {
        self.to_string()
    }

    fn repr(&self) -> String {
        self.to_string()
    }

    fn type_name(&self) -> String {
        return "int".into()
    }

    fn add(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Int(self + n)),
            _ => None,
        }
    }

    fn sub(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Int(self - n)),
            _ => None,
        }
    }

    fn mul(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Int(self * n)),
            _ => None,
        }
    }

    fn div(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Int(self / n)),
            _ => None,
        }
    }
}

impl RispType for Float {
    fn display(&self) -> String {
        format!("{self:?}")
    }

    fn repr(&self) -> String {
        format!("{self:?}")
    }

    fn type_name(&self) -> String {
        return "float".into()
    }

    fn add(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Float(self + f64::from(*n))),
            Type::Float(f) => Some(Type::Float(self + f)),
            _ => None,
        }
    }

    fn sub(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Float(self - f64::from(*n))),
            Type::Float(f) => Some(Type::Float(self - f)),
            _ => None,
        }
    }

    fn mul(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Float(self * f64::from(*n))),
            Type::Float(f) => Some(Type::Float(self * f)),
            _ => None,
        }
    }

    fn div(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Float(self / f64::from(*n))),
            Type::Float(f) => Some(Type::Float(self / f)),
            _ => None,
        }
    }

    fn radd(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Float(self + f64::from(*n))),
            _ => None,
        }
    }

    fn rsub(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Float(f64::from(*n) - self)),
            _ => None,
        }
    }

    fn rmul(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Float(self * f64::from(*n))),
            _ => None,
        }
    }

    fn rdiv(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Float(f64::from(*n) / self)),
            _ => None,
        }
    }
}

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
        match other {
            Type::Str(s) => Some(Type::Str(self.clone() + &s)),
            _ => None,
        }
    }

    fn mul(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Str(self.repeat(*n as usize))),
            _ => None,
        }
    }

    fn rmul(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Str(self.repeat(*n as usize))),
            _ => None,
        }
    }
}

impl RispType for List {
    fn repr(&self) -> String {
        let mut iter = self.iter();

        let mut result = match iter.next() {
            Some(el) => {
                let r = el.repr();
                format!(", {r}")
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
        match other {
            Type::List(el) => Some(Type::List(
                self.iter().cloned().chain(el.iter().cloned()).collect(),
            )),
            _ => None,
        }
    }

    fn mul(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::List(
                self.iter()
                    .cloned()
                    .cycle()
                    .take(self.len() * *n as usize)
                    .collect(),
            )),
            _ => None,
        }
    }

    fn rmul(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::List(
                self.iter()
                    .cloned()
                    .cycle()
                    .take(self.len() * *n as usize)
                    .collect(),
            )),
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
