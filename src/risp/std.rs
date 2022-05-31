mod types;
use types::{ Type, RispType };
use std::{io, io::prelude::*};

/// Prints values to STDOUT, without a trailing newline
#[no_mangle]
pub extern fn print(_in: Vec<Type>) -> Vec<Type> {
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
#[no_mangle]
pub extern fn println(_in: Vec<Type>) -> Vec<Type> {
    print(_in);
    print!("\n");

    return Vec::new();
}

#[no_mangle]
pub extern fn input(_in: Vec<Type>) -> Vec<Type> {
    print(_in);

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    return vec!(Type::Str(buffer.trim_end().to_owned()))
}