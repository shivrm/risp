pub type Int = i32;
pub type Str = String;
pub type List = Vec<Type>;
pub type RustFn = fn(List) -> List;
pub struct Null;

#[derive(Clone, Copy)]
pub enum Op {
    Plus,
    Minus,
    Star,
    Slash
}

macro_rules! delegate {
    ($obj:ident, $name:ident, $( $x:expr ),*) => {
        match $obj {
            Type::Int(n)    => n.$name($($x)*),
            Type::Str(s)    => s.$name($($x)*),
            Type::List(l)   => l.$name($($x)*),
            Type::RustFn(f) => f.$name($($x)*),
            Type::Operator(op) => op.$name($($x)*),
            Type::Null      => Null.$name($($x)*),
        }
    };
}

#[derive(Clone)]
pub enum Type {
    Int(Int),
    Str(Str),
    List(List),
    RustFn(RustFn),
    Operator(Op),
    Null,
}

pub trait RispType {
    fn repr(&self) -> String;
    fn display(&self) -> String;
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

    fn add(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Int(self + n)),
            _            => None
        }
    }

    fn sub(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Int(self - n)),
            _            => None
        }
    }

    fn mul(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Int(self * n)),
            _            => None
        }
    }

    fn div(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Int(self / n)),
            _            => None
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

    fn add(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Str(s) => Some(Type::Str(self.clone() + &s)),
            _            => None
        }   
    }

    fn mul(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Str(self.repeat(*n as usize))),
            _            => None
        }
    }

    fn rmul(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::Str(self.repeat(*n as usize))),
            _            => None
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
            },
            None => String::from("[")
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
            None => String::from("[")
        };

        for el in iter {
            result += &format!(", {}", el.display())
        }

        result.push(']');

        result
    }

    fn add(&self, other: &Type) -> Option<Type> {
        match other {
            Type::List(el) => Some(Type::List(self.iter().cloned().chain(el.iter().cloned()).collect())),
            _              => None
        }
    }

    fn mul(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::List(self.iter().cloned().cycle().take(self.len() * *n as usize).collect())),
            _            => None
        }
    }

    fn rmul(&self, other: &Type) -> Option<Type> {
        match other {
            Type::Int(n) => Some(Type::List(self.iter().cloned().cycle().take(self.len() * *n as usize).collect())),
            _            => None
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
}

impl RispType for Op {
    fn display(&self) -> String {
        let value = match self {
            Op::Plus => "+",
            Op::Minus => "-",
            Op::Star => "*",
            Op::Slash => "/"
        };
        value.to_owned()
    }

    fn repr(&self) -> String {
        self.display()
    }
}

impl RispType for Null {
    fn display(&self) -> String {
        "Null".to_owned()
    }

    fn repr(&self) -> String {
        String::new()
    }
}