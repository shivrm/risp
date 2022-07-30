use std::collections::HashMap;
use std::{io, io::prelude::*};

use super::{RuntimeError, Type, WrappedType};

/// Prints values to STDOUT, without a trailing newline
pub fn print(_in: Vec<WrappedType>) -> Result<Vec<WrappedType>, RuntimeError> {
    let mut iter = _in.iter();

    match iter.next() {
        Some(v) => print!("{}", v.display()),
        None => (()),
    }

    for el in iter {
        print!(" {}", el.display());
    }

    io::stdout().flush().unwrap();

    Ok(Vec::new())
}

/// Prints values to STDOUT, followed by a newline
pub fn println(_in: Vec<WrappedType>) -> Result<Vec<WrappedType>, RuntimeError> {
    print(_in)?;
    print!("\n");

    Ok(Vec::new())
}

pub fn input(_in: Vec<WrappedType>) -> Result<Vec<WrappedType>, RuntimeError> {
    print(_in)?;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    Ok(vec![WrappedType::Str(buffer.trim_end().to_owned())])
}

lazy_static! {
    pub static ref SYMBOLS: HashMap<String, WrappedType> = {
        let mut h = HashMap::new();
        h.insert("println".into(), WrappedType::RustFn(println));
        h.insert("print".into(), WrappedType::RustFn(print));
        h.insert("input".into(), WrappedType::RustFn(input));
        h
    };
}
