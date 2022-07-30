use crate::risp::{Op};

mod misc;
mod num;

pub use misc::*; pub use num::*;

macro_rules! derive_from {
    ($($kind:ident),*) => {
        $(
        impl From<$kind> for Type {
            fn from(item: $kind) -> Self {
                Type::$kind(item)
            }
        }
        )*
    };
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
    fn eq(&self, _other: &Type) -> Option<bool> {
        None
    }
    fn gt(&self, _other: &Type) -> Option<bool> {
        None
    }
    fn lt(&self, _other: &Type) -> Option<bool> {
        None
    }
}

#[derive(Clone)]
pub enum Type {
    Int(Int),
    Bool(Bool),
    Float(Float),
    Str(Str),
    List(List),
    RustFn(RustFn),
    RustMacro(RustMacro),
    Operator(Op),
    Null,
}

impl Type {
    fn unwrap(&self) -> Box<&dyn RispType> {
        use Type::*;
        match self {
            Int(i) => Box::new(i),
            Bool(b) => Box::new(b),
            Float(f) => Box::new(f),
            Str(s) => Box::new(s),
            List(l) => Box::new(l),
            RustFn(f) => Box::new(f),
            RustMacro(m) => Box::new(m),
            Operator(op) => Box::new(op),
            Null => Box::new(&misc::Null),
        }
    }

    pub fn type_name(&self) -> String {
        use Type::*;
        match self {
            Int(_) => "int",
            Bool(_) => "bool",
            Float(_) => "float",
            Str(_) => "str",
            List(_) => "list",
            RustFn(_) => "rustfn",
            RustMacro(_) => "rustmacro",
            Operator(_) => "operator",
            Null => "null",
        }.into()
    }
}

derive_from!(Int, Bool, Float, Str, List, RustFn, RustMacro);

// Function definitions could be done by a macro 🤔
impl RispType for Type {
    fn repr(&self) -> String {
        self.unwrap().repr()
    }
    
    fn display(&self) -> String {
        self.unwrap().repr()
    }

    fn add(&self, other: &Type) -> Option<Type> {
        self.unwrap().add(other)
    }

    fn sub(&self, other: &Type) -> Option<Type> {
        self.unwrap().sub(other)
    }

    fn mul(&self, other: &Type) -> Option<Type> {
        self.unwrap().mul(other)
    }

    fn div(&self, other: &Type) -> Option<Type> {
        self.unwrap().div(other)
    }

    fn radd(&self, other: &Type) -> Option<Type> {
        self.unwrap().radd(other)
    }

    fn rsub(&self, other: &Type) -> Option<Type> {
        self.unwrap().rsub(other)
    }

    fn rmul(&self, other: &Type) -> Option<Type> {
        self.unwrap().rmul(other)
    }

    fn rdiv(&self, other: &Type) -> Option<Type> {
        self.unwrap().rdiv(other)
    }

    fn eq(&self, other: &Type) -> Option<bool> {
        self.unwrap().eq(other)
    }

    fn gt(&self, other: &Type) -> Option<bool> {
        self.unwrap().gt(other)
    }

    fn lt(&self, other: &Type) -> Option<bool> {
        self.unwrap().lt(other)
    }
}
