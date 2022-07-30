use super::{Type, RispType};

pub type Int = i32;
pub type Bool = bool;
pub type Float = f64;

impl RispType for Int {
    

    fn display(&self) -> String {
        self.to_string()
    }

    fn repr(&self) -> String {
        self.to_string()
    }

    fn add(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => (self + n).into(),
            _ => return None,
        };
        Some(res)
    }

    fn sub(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => (self - n).into(),
            _ => return None,
        };
        Some(res)
    }

    fn mul(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => (self * n).into(),
            _ => return None,
        };
        Some(res)
    }

    fn div(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => (self / n).into(),
            _ => return None,
        };
        Some(res)
    }
    
    fn equal(&self, other: &Type) -> Option<bool> {
        let res = match other {
            Type::Int(n) => (self == n),
            _ => return None,
        };
        Some(res)
    }

    fn greater(&self, other: &Type) -> Option<bool> {
        let res = match other {
            Type::Int(n) => (self > n),
            _ => return None,
        };
        Some(res)
    }

    fn less(&self, other: &Type) -> Option<bool> {
        let res = match other {
            Type::Int(n) => (self < n),
            _ => return None,
        };
        Some(res)
    }
}

impl RispType for Bool {
    

    fn display(&self) -> String {
        self.to_string()
    }

    fn repr(&self) -> String {
        self.to_string()
    }

    fn add(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Bool(b) => (*self as i32 + *b as i32).into(),
            Type::Int(n) => (*self as i32 + n).into(),
            Type::Float(f) => ((*self as i32 as f64) + f).into(),
            _ => return None,
        };
        Some(res)
    }

    fn sub(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Bool(b) => (*self as i32 - *b as i32).into(),
            Type::Int(n) => (*self as i32 - n).into(),
            Type::Float(f) => ((*self as i32 as f64) - f).into(),
            _ => return None,
        };
        Some(res)
    }

    fn mul(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Bool(b) => (*self as i32 * *b as i32).into(),
            Type::Int(n) => (*self as i32 * n).into(),
            Type::Float(f) => ((*self as i32 as f64) * f).into(),
            _ => return None,
        };
        Some(res)
    }

    fn div(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Bool(b) => (*self as i32 / *b as i32).into(),
            Type::Int(n) => (*self as i32 / n).into(),
            Type::Float(f) => ((*self as i32 as f64) / f).into(),
            _ => return None,
        };
        Some(res)
    }

    fn radd(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => (n + *self as i32).into(),
            Type::Float(f) => (f + (*self as i32 as f64)).into(),
            _ => return None,
        };
        Some(res)
    }

    fn rsub(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => (n - *self as i32).into(),
            Type::Float(f) => (f - (*self as i32 as f64)).into(),
            _ => return None,
        };
        Some(res)
    }

    fn rmul(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => (n * *self as i32).into(),
            Type::Float(f) => (f * (*self as i32 as f64)).into(),
            _ => return None,
        };
        Some(res)
    }

    fn rdiv(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => (n / *self as i32).into(),
            Type::Float(f) => (f / (*self as i32 as f64)).into(),
            _ => return None,
        };
        Some(res)
    }

    fn equal(&self, other: &Type) -> Option<bool> {
        let res = match other {
            Type::Bool(b) => (self == b),
            &Type::Int(n) => ((*self as i32) == n),
            &Type::Float(f) => ((*self as i32 as f64) == f),
            _ => return None,
        };
        Some(res)
    }

    fn greater(&self, other: &Type) -> Option<bool> {
        let res = match other {
            Type::Bool(b) => (self > b),
            &Type::Int(n) => ((*self as i32) > n),
            &Type::Float(f) => ((*self as i32 as f64) > f),
            _ => return None,
        };
        Some(res)
    }

    fn less(&self, other: &Type) -> Option<bool> {
        let res = match other {
            Type::Bool(b) => (self < b),
            &Type::Int(n) => ((*self as i32) < n),
            &Type::Float(f) => ((*self as i32 as f64) < f),
            _ => return None,
        };
        Some(res)
    }
}

impl RispType for Float {
    

    fn display(&self) -> String {
        format!("{self:?}")
    }

    fn repr(&self) -> String {
        format!("{self:?}")
    }

    fn add(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => (self + *n as f64).into(),
            Type::Float(f) => (self + f).into(),
            _ => return None,
        };
        Some(res)
    }

    fn sub(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => (self - *n as f64).into(),
            Type::Float(f) => (self - f).into(),
            _ => return None,
        };
        Some(res)
    }

    fn mul(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => (self * *n as f64).into(),
            Type::Float(f) => (self * f).into(),
            _ => return None,
        };
        Some(res)
    }

    fn div(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => (self / *n as f64).into(),
            Type::Float(f) => (self / f).into(),
            _ => return None,
        };
        Some(res)
    }

    fn radd(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => (*n as f64 + self).into(),
            _ => return None,
        };
        Some(res)
    }

    fn rsub(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => (*n as f64 - self).into(),
            _ => return None,
        };
        Some(res)
    }

    fn rmul(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => (*n as f64 * self).into(),
            _ => return None,
        };
        Some(res)
    }

    fn rdiv(&self, other: &Type) -> Option<Type> {
        let res = match other {
            Type::Int(n) => (*n as f64 / self).into(),
            _ => return None,
        };
        Some(res)
    }

    fn equal(&self, other: &Type) -> Option<bool> {
        let res = match other {
            &Type::Int(n) => (*self == (n as f64)),
            Type::Float(f) => (self == f),
            _ => return None,
        };
        Some(res)
    }

    fn greater(&self, other: &Type) -> Option<bool> {
        let res = match other {
            &Type::Int(n) => (*self > (n as f64)),
            Type::Float(f) => (self > f),
            _ => return None,
        };
        Some(res)
    }

    fn less(&self, other: &Type) -> Option<bool> {
        let res = match other {
            &Type::Int(n) => (*self < (n as f64)),
            Type::Float(f) => (self < f),
            _ => return None,
        };
        Some(res)
    }
}
