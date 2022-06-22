use std::{env, fs};
use std::{io, io::prelude::*};

mod risp;
use risp::{AstNode, RispType, Type};

mod lexspeed;

#[macro_use]
extern crate lazy_static;

/// Interprets multiple expressions using the same interpreter
fn interpret_exprs(interpreter: &mut risp::Interpreter, asts: Vec<AstNode>, output: bool) {
    for ast in asts.iter().cloned() {
        let value = interpreter.eval(ast);

        match value {
            Ok(Type::Null) => (),
            Ok(v) if output => println!("{}", v.repr()),
            Err(err) => eprintln!("{err:?}"),
            _ => ()
        }
    }
}

fn repl() {
    let mut interpreter = risp::Interpreter::new();

    println!("risp v0.6.0. Type 'quit' to quit");

    // REPL loop
    loop {
        print!("> ");
        io::stdout().flush().expect("Could not flush buffer");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        match line.trim() {
            "quit" => break,

            // Interprets all expressions on the line
            _ => match risp::to_ast(&mut line) {
                Ok(ast) => interpret_exprs(&mut interpreter, ast, true),
                Err(err) => eprintln!("{err:?}"),
            },
        }
    }
}

/// Runs a file containing RISP code
fn run_file(filename: &str) {
    let mut src = fs::read_to_string(filename).expect("Could not open file");

    let mut interpreter = risp::Interpreter::new();

    match risp::to_ast(&mut src) {
        Ok(asts) => interpret_exprs(&mut interpreter, asts, false),
        Err(err) => eprintln!("{err:?}"),
    }
}

fn main() {
    match env::args().nth(1).as_deref() {
        None => repl(),
        Some("bench") => lexspeed::lex_speed(),
        Some(filename) => run_file(&filename),
    }
}
