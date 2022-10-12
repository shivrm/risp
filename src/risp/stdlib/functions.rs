use std::collections::HashMap;
use std::{io, io::prelude::*};

use crate::risp::{RuntimeError, Value, Interpreter};

/// Prints values to STDOUT, without a trailing newline
pub fn print(_: &mut Interpreter, _in: Vec<Value>) -> Result<Vec<Value>, RuntimeError> {
    let mut iter = _in.iter();

    match iter.next() {
        Some(v) => print!("{}", v.display()),
        None => (),
    }

    for el in iter {
        print!(" {}", el.display());
    }

    io::stdout().flush().unwrap();

    Ok(Vec::new())
}

/// Prints values to STDOUT, followed by a newline
pub fn println(inter: &mut Interpreter, _in: Vec<Value>) -> Result<Vec<Value>, RuntimeError> {
    print(inter, _in)?;
    print!("\n");

    Ok(Vec::new())
}

pub fn input(inter: &mut Interpreter, _in: Vec<Value>) -> Result<Vec<Value>, RuntimeError> {
    print(inter, _in)?;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    Ok(vec![Value::Str(buffer.trim_end().to_owned())])
}

lazy_static! {
    pub static ref SYMBOLS: HashMap<String, Value> = {
        let mut h = HashMap::new();
        h.insert("println".into(), Value::RustFn(println));
        h.insert("print".into(), Value::RustFn(print));
        h.insert("input".into(), Value::RustFn(input));
        h
    };
}
