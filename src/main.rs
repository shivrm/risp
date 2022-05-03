use std::io;
use std::io::prelude::*;

mod risp;

fn main() {
    // Initial greeting
    print!(concat!(
        "risp v0.3.0\n",
        "Type 'bugs' or 'copyright' for more information.\n",
        "Type 'q' or 'quit' to quit\n"
    ));
    
    let interpreter = risp::Intepreter::new();

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
            _ => {
                match risp::to_ast(&mut line) {
                    Ok(ast) => {
                        let value = interpreter.eval(ast);

                        match value {
                            Ok(v) => println!("\x1b[32m{v}\x1b[0m"),
                            Err(err) => eprintln!("\x1b[33m{err}\x1b[0m")
                        }
                    },
                    Err(err) => eprintln!("\x1b[33m{err}\x1b[0m")
                }
            }
        }

    }
}