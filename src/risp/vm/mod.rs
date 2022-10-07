mod interpreter;
mod types;

use std::fmt;

pub use interpreter::Interpreter;
pub use types::Value;

#[derive(Debug)]
pub enum ErrorKind {
    NameError,
    TypeError,
    ValueError,
}

pub struct RuntimeError {
    pub kind: ErrorKind,
    pub msg: String,
}

impl fmt::Debug for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.msg)
    }
}
