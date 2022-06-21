use std::time::{Duration, Instant};
use std::{env, fs};
use std::{io, io::prelude::*};

mod risp;
use risp::{AstNode, RispType, Type};

use crate::risp::TokenKind;

#[macro_use]
extern crate lazy_static;

/// Interprets multiple expressions using the same interpreter
fn interpret_exprs(interpreter: &mut risp::Interpreter, asts: Vec<AstNode>) {
    for ast in asts.iter().cloned() {
        let value = interpreter.eval(ast);

        match value {
            Ok(Type::Null) => (),
            Ok(v) => println!("\x1b[32m{}\x1b[0m", v.repr()),
            Err(err) => eprintln!("\x1b[33m{err:?}\x1b[0m"),
        }
    }
}

fn repl() {
    let mut interpreter = risp::Interpreter::new();

    // Initial greeting
    print!(concat!("risp v0.5.0\n", "Type 'quit' to quit\n"));

    // REPL loop
    loop {
        print!("> ");
        io::stdout().flush().expect("Could not flush buffer");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        match &line.trim()[..] {
            "quit" => break,

            // Interprets all expressions on the line
            _ => match risp::to_ast(&mut line) {
                Ok(ast) => {
                    interpret_exprs(&mut interpreter, ast);
                }
                Err(err) => eprintln!("\x1b[33m{err:?}\x1b[0m"),
            },
        }
    }
}

/// Runs a file containing RISP code
fn run_file(filename: &str) {
    let mut src = fs::read_to_string(filename).expect("Could not open file");

    let mut interpreter = risp::Interpreter::new();

    match risp::to_ast(&mut src) {
        Ok(asts) => {
            for ast in asts.iter().cloned() {
                match interpreter.eval(ast) {
                    Ok(_) => (()),
                    Err(err) => {
                        eprintln!("\x1b[33m{err:?}\x1b[0m");
                        break;
                    }
                }
            }
        }
        Err(err) => eprintln!("\x1b[33m{err:?}\x1b[0m"),
    }
}

/// Used to benchmark a function
fn bench(mut func: impl FnMut(), num: u32, samples: u32) -> Duration {
    let mut max = Duration::from_secs(0);
    let mut min = Duration::from_secs(u64::MAX);
    let mut tot = Duration::from_secs(0);
    for _ in 0..samples {
        let time = Instant::now();
        for _ in 0..num {
            func();
        }
        let elapsed = time.elapsed();
        if elapsed > max {
            max = elapsed;
        }
        if elapsed < min {
            min = elapsed;
        }
        tot += elapsed;
    }
    println!("Total elapsed: {:?}", tot);
    let avg = (tot / 100) / num;
    println!(
        "[max: {:?}, min: {:?}, avg: {:?}]",
        max / num,
        min / num,
        avg
    );
    return avg;
}

/// Wrapper function for `benchmark`. Tests the speed of the lexer
fn lex_speed() {
    let src =
        fs::read_to_string("scripts/example.risp").expect("Could not open scripts/example.risp");

    let bench_fn = || {
        let mut lexer = risp::Lexer::new(&src);
        while lexer.next().unwrap().kind == TokenKind::EOF { /* Benchmark */ }
    };

    let avg = bench(bench_fn, 5000, 100);

    let bytes_per_sec = (1_000_000_000.0 / avg.as_nanos() as f64) * src.len() as f64;
    let mb = 1000.0 * 1000.0;
    let mib = 1024.0 * 1024.0;

    println!(
        "Average lex speed: {:.3} MB/s => {:.3} MiB/s",
        bytes_per_sec / mb,
        bytes_per_sec / mib
    );
}

fn main() {
    match env::args().nth(1).as_deref() {
        None => repl(),
        Some("bench") => lex_speed(),
        Some(filename) => run_file(&filename),
    }
}
