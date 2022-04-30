use std::io;
use std::io::prelude::*;

mod risp;

fn main() {
    print!(concat!(
        "risp v0.2.0\n",
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
            _ => {
                match risp::eval(&mut line) {
                    Ok(value) => println!("{value}"),
                    Err(err) => eprintln!("{err}")
                }
            }
        }

    }
}