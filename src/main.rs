use crate::risp::Token;
use std::io;
use std::io::prelude::*;
use std::time::{Duration, Instant};

mod risp;
use risp::RispType;

fn repl() {
    let interpreter = risp::Intepreter::new();
    
    // Initial greeting
    print!(concat!(
        "risp v0.3.0\n",
        "Type 'bugs' or 'copyright' for more information.\n",
        "Type 'q' or 'quit' to quit\n"
    ));

    loop {
        print!(">>> ");
        io::stdout().flush().expect("Could not flush buffer");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        match &line.trim()[..] {
            "bugs" => println!("Report bugs at https://github.com/shivrm/risp/issues"),
            "copyright" => println!("Copyright (c) 2022 shivrm"),
            "quit" | "q" => {
                println!("Quitting");
                break;
            }
            _ => match risp::to_ast(&mut line) {
                Ok(ast) => {
                    let value = interpreter.eval(ast);

                    match value {
                        Ok(v) => println!("\x1b[32m{}\x1b[0m", v.repr()),
                        Err(err) => eprintln!("\x1b[33m{err}\x1b[0m"),
                    }
                }
                Err(err) => eprintln!("\x1b[33m{err}\x1b[0m"),
            },
        }
    }
}

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

fn lex_speed() {
    let src = "(println 1 2)";

    let bench_fn = || {
        let mut lexer = risp::Lexer::new(src);
        while !matches!(
            lexer.next(),
            Ok(Token {
                kind: risp::Kind::EOF,
                ..
            })
        ) { /* Benchmark */ }
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
    // TODO: Change this and everything else to how you'd like it
    // lex_speed();
    repl();
}
