use super::{WrappedType, Type};

pub type Int = i32;
pub type Bool = bool;
pub type Float = f64;

impl Type for Int {
    fn display(&self) -> String {
        self.to_string()
    }

    fn repr(&self) -> String {
        self.to_string()
    }

    fn add(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => (self + n).into(),
            _ => return None,
        };
        Some(res)
    }

    fn sub(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => (self - n).into(),
            _ => return None,
        };
        Some(res)
    }

    fn mul(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => (self * n).into(),
            _ => return None,
        };
        Some(res)
    }

    fn div(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => (self / n).into(),
            _ => return None,
        };
        Some(res)
    }
    
    fn eq(&self, other: &WrappedType) -> Option<bool> {
        let res = match other {
            WrappedType::Int(n) => (self == n),
            _ => return None,
        };
        Some(res)
    }

    fn gt(&self, other: &WrappedType) -> Option<bool> {
        let res = match other {
            WrappedType::Int(n) => (self > n),
            _ => return None,
        };
        Some(res)
    }

    fn lt(&self, other: &WrappedType) -> Option<bool> {
        let res = match other {
            WrappedType::Int(n) => (self < n),
            _ => return None,
        };
        Some(res)
    }
}

impl Type for Bool {
    

    fn display(&self) -> String {
        self.to_string()
    }

    fn repr(&self) -> String {
        self.to_string()
    }

    fn add(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Bool(b) => (*self as i32 + *b as i32).into(),
            WrappedType::Int(n) => (*self as i32 + n).into(),
            WrappedType::Float(f) => ((*self as i32 as f64) + f).into(),
            _ => return None,
        };
        Some(res)
    }

    fn sub(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Bool(b) => (*self as i32 - *b as i32).into(),
            WrappedType::Int(n) => (*self as i32 - n).into(),
            WrappedType::Float(f) => ((*self as i32 as f64) - f).into(),
            _ => return None,
        };
        Some(res)
    }

    fn mul(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Bool(b) => (*self as i32 * *b as i32).into(),
            WrappedType::Int(n) => (*self as i32 * n).into(),
            WrappedType::Float(f) => ((*self as i32 as f64) * f).into(),
            _ => return None,
        };
        Some(res)
    }

    fn div(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Bool(b) => (*self as i32 / *b as i32).into(),
            WrappedType::Int(n) => (*self as i32 / n).into(),
            WrappedType::Float(f) => ((*self as i32 as f64) / f).into(),
            _ => return None,
        };
        Some(res)
    }

    fn radd(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => (n + *self as i32).into(),
            WrappedType::Float(f) => (f + (*self as i32 as f64)).into(),
            _ => return None,
        };
        Some(res)
    }

    fn rsub(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => (n - *self as i32).into(),
            WrappedType::Float(f) => (f - (*self as i32 as f64)).into(),
            _ => return None,
        };
        Some(res)
    }

    fn rmul(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => (n * *self as i32).into(),
            WrappedType::Float(f) => (f * (*self as i32 as f64)).into(),
            _ => return None,
        };
        Some(res)
    }

    fn rdiv(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => (n / *self as i32).into(),
            WrappedType::Float(f) => (f / (*self as i32 as f64)).into(),
            _ => return None,
        };
        Some(res)
    }

    fn eq(&self, other: &WrappedType) -> Option<bool> {
        let res = match other {
            WrappedType::Bool(b) => (self == b),
            &WrappedType::Int(n) => ((*self as i32) == n),
            &WrappedType::Float(f) => ((*self as i32 as f64) == f),
            _ => return None,
        };
        Some(res)
    }

    fn gt(&self, other: &WrappedType) -> Option<bool> {
        let res = match other {
            WrappedType::Bool(b) => (self > b),
            &WrappedType::Int(n) => ((*self as i32) > n),
            &WrappedType::Float(f) => ((*self as i32 as f64) > f),
            _ => return None,
        };
        Some(res)
    }

    fn lt(&self, other: &WrappedType) -> Option<bool> {
        let res = match other {
            WrappedType::Bool(b) => (self < b),
            &WrappedType::Int(n) => ((*self as i32) < n),
            &WrappedType::Float(f) => ((*self as i32 as f64) < f),
            _ => return None,
        };
        Some(res)
    }
}

impl Type for Float {
    

    fn display(&self) -> String {
        format!("{self:?}")
    }

    fn repr(&self) -> String {
        format!("{self:?}")
    }

    fn add(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => (self + *n as f64).into(),
            WrappedType::Float(f) => (self + f).into(),
            _ => return None,
        };
        Some(res)
    }

    fn sub(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => (self - *n as f64).into(),
            WrappedType::Float(f) => (self - f).into(),
            _ => return None,
        };
        Some(res)
    }

    fn mul(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => (self * *n as f64).into(),
            WrappedType::Float(f) => (self * f).into(),
            _ => return None,
        };
        Some(res)
    }

    fn div(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => (self / *n as f64).into(),
            WrappedType::Float(f) => (self / f).into(),
            _ => return None,
        };
        Some(res)
    }

    fn radd(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => (*n as f64 + self).into(),
            _ => return None,
        };
        Some(res)
    }

    fn rsub(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => (*n as f64 - self).into(),
            _ => return None,
        };
        Some(res)
    }

    fn rmul(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => (*n as f64 * self).into(),
            _ => return None,
        };
        Some(res)
    }

    fn rdiv(&self, other: &WrappedType) -> Option<WrappedType> {
        let res = match other {
            WrappedType::Int(n) => (*n as f64 / self).into(),
            _ => return None,
        };
        Some(res)
    }

    fn eq(&self, other: &WrappedType) -> Option<bool> {
        let res = match other {
            &WrappedType::Int(n) => (*self == (n as f64)),
            WrappedType::Float(f) => (self == f),
            _ => return None,
        };
        Some(res)
    }

    fn gt(&self, other: &WrappedType) -> Option<bool> {
        let res = match other {
            &WrappedType::Int(n) => (*self > (n as f64)),
            WrappedType::Float(f) => (self > f),
            _ => return None,
        };
        Some(res)
    }

    fn lt(&self, other: &WrappedType) -> Option<bool> {
        let res = match other {
            &WrappedType::Int(n) => (*self < (n as f64)),
            WrappedType::Float(f) => (self < f),
            _ => return None,
        };
        Some(res)
    }
}
