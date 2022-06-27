use crate::risp::{Op};

mod misc;
mod num;

pub use misc::*; pub use num::*;

macro_rules! delegate {
    ($obj:ident, $name:ident, $( $x:expr ),*) => {
        match $obj {
            Type::Int(n)    => n.$name($($x)*),
            Type::Bool(b)   => b.$name($($x)*),
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

derive_from!(Int, Bool, Float, Str, List, RustFn, RustMacro);

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
