use std::{io, io::prelude::*};
use std::collections::HashMap;

use crate::risp::types::{ Type, RispType };

/// Prints values to STDOUT, without a trailing newline
pub fn print(_in: Vec<Type>) -> Vec<Type> {
    let mut iter = _in.iter();

    match iter.next() {
        Some(v) => print!("{}", v.display()),
        None => (())
    }
    
    for el in iter {
        print!(" {}", el.display());
    }

    io::stdout().flush().unwrap();

    return Vec::new();
}

/// Prints values to STDOUT, followed by a newline
pub fn println(_in: Vec<Type>) -> Vec<Type> {
    print(_in);
    print!("\n");

    return Vec::new();
}

pub fn input(_in: Vec<Type>) -> Vec<Type> {
    print(_in);

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    return vec!(Type::Str(buffer.trim_end().to_owned()))
}

type RustFn = fn(Vec<Type>) -> Vec<Type>;

lazy_static!(
    static ref SYMBOLS: HashMap<&'static str, &'static RustFn> = {
        let mut h = HashMap::new();
        h.insert("println", &(println as RustFn));
        h.insert("print", &(print as RustFn));
        h.insert("input", &(input as RustFn));
        h
    };
);