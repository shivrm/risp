pub type Int = i32;
pub type Str = String;
pub type List = Vec<Type>;
pub type RustFn = fn(List) -> List;
pub struct Null;

macro_rules! delegate {
    ($obj:ident, $name:ident, $( $x:expr ),*) => {
        match $obj {
            Type::Int(n)    => n.$name($($x)*),
            Type::Str(s)    => s.$name($($x)*),
            Type::List(l)   => l.$name($($x)*),
            Type::RustFn(f) => f.$name($($x)*),
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
    Null,
}

pub trait RispPrint {
    fn repr(&self) -> String;
    fn display(&self) -> String;
    fn add(&self, other: Type) -> Option<Type>;
    fn sub(&self, other: Type) -> Option<Type>;
    fn mul(&self, other: Type) -> Option<Type>;
    fn div(&self, other: Type) -> Option<Type>;
}

// Function definitions could be done by a macro 🤔
impl RispPrint for Type {
    fn repr(&self) -> String {
        delegate!(self, repr,)
    }

    fn display(&self) -> String {
        delegate!(self, display,)
    }

    fn add(&self, other: Type) -> Option<Type> {
        delegate!(self, add, other)
    }

    fn sub(&self, other: Type) -> Option<Type> {
        delegate!(self, sub, other)
    }

    fn mul(&self, other: Type) -> Option<Type> {
        delegate!(self, mul, other)
    }

    fn div(&self, other: Type) -> Option<Type> {
        delegate!(self, div, other)
    }
}


impl RispPrint for Int {
    fn display(&self) -> String {
        self.to_string()
    }

    fn repr(&self) -> String {
        self.to_string()
    }
}

impl RispPrint for Str {
    fn display(&self) -> String {
        self.clone()
    }

    fn repr(&self) -> String {
        format!("{self:?}")
    }
}

impl RispPrint for List {
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
}

impl RispPrint for RustFn {
    fn display(&self) -> String {
        "<Rust Function>".to_owned()
    }

    fn repr(&self) -> String {
        "<Rust Function>".to_owned()
    }
}

impl RispPrint for Null {
    fn display(&self) -> String {
        "Null".to_owned()
    }

    fn repr(&self) -> String {
        String::new()
    }
}