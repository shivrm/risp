use crate::risp::Op;

mod misc;
mod num;

pub use misc::*;
pub use num::*;

macro_rules! derive_from {
    ($($kind:ident),*) => {
        $(
        impl From<$kind> for WrappedType {
            fn from(item: $kind) -> Self {
                WrappedType::$kind(item)
            }
        }
        )*
    };
}

pub trait Type {
    fn repr(&self) -> String;
    fn display(&self) -> String;

    fn add(&self, _other: &WrappedType) -> Option<WrappedType> {
        None
    }
    fn sub(&self, _other: &WrappedType) -> Option<WrappedType> {
        None
    }
    fn mul(&self, _other: &WrappedType) -> Option<WrappedType> {
        None
    }
    fn div(&self, _other: &WrappedType) -> Option<WrappedType> {
        None
    }
    fn radd(&self, _other: &WrappedType) -> Option<WrappedType> {
        None
    }
    fn rsub(&self, _other: &WrappedType) -> Option<WrappedType> {
        None
    }
    fn rmul(&self, _other: &WrappedType) -> Option<WrappedType> {
        None
    }
    fn rdiv(&self, _other: &WrappedType) -> Option<WrappedType> {
        None
    }
    fn eq(&self, _other: &WrappedType) -> Option<bool> {
        None
    }
    fn gt(&self, _other: &WrappedType) -> Option<bool> {
        None
    }
    fn lt(&self, _other: &WrappedType) -> Option<bool> {
        None
    }
}

#[derive(Clone)]
pub enum WrappedType {
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

impl WrappedType {
    fn unwrap(&self) -> Box<&dyn Type> {
        use WrappedType::*;
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
        use WrappedType::*;
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
        }
        .into()
    }
}

derive_from!(Int, Bool, Float, Str, List, RustFn, RustMacro);

// Function definitions could be done by a macro 🤔
impl Type for WrappedType {
    fn repr(&self) -> String {
        self.unwrap().repr()
    }

    fn display(&self) -> String {
        self.unwrap().repr()
    }

    fn add(&self, other: &WrappedType) -> Option<WrappedType> {
        self.unwrap().add(other)
    }

    fn sub(&self, other: &WrappedType) -> Option<WrappedType> {
        self.unwrap().sub(other)
    }

    fn mul(&self, other: &WrappedType) -> Option<WrappedType> {
        self.unwrap().mul(other)
    }

    fn div(&self, other: &WrappedType) -> Option<WrappedType> {
        self.unwrap().div(other)
    }

    fn radd(&self, other: &WrappedType) -> Option<WrappedType> {
        self.unwrap().radd(other)
    }

    fn rsub(&self, other: &WrappedType) -> Option<WrappedType> {
        self.unwrap().rsub(other)
    }

    fn rmul(&self, other: &WrappedType) -> Option<WrappedType> {
        self.unwrap().rmul(other)
    }

    fn rdiv(&self, other: &WrappedType) -> Option<WrappedType> {
        self.unwrap().rdiv(other)
    }

    fn eq(&self, other: &WrappedType) -> Option<bool> {
        self.unwrap().eq(other)
    }

    fn gt(&self, other: &WrappedType) -> Option<bool> {
        self.unwrap().gt(other)
    }

    fn lt(&self, other: &WrappedType) -> Option<bool> {
        self.unwrap().lt(other)
    }
}
