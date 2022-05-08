use crate::risp::Type;
use std::{io, io::prelude::*};

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

    return vec!(Type::String(buffer.trim_end().to_owned()))
}

pub fn get_name(name: &str) -> Option<Type> {
    let value = match name {
        "print" => Type::BuiltinFn(&print),
        "println" => Type::BuiltinFn(&println),
        "input" => Type::BuiltinFn(&input),
        _ => return None
    };

    return Some(value)
}