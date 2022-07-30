use crate::risp::{Lexer, TokenKind};
use std::fs;
use std::time::{Duration, Instant};

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
pub fn lex_speed() {
    let src =
        fs::read_to_string("scripts/example.risp").expect("Could not open scripts/example.risp");

    // Function to benchmark. Creates a lexer and lexes the entire source file each time
    // Source file must not contain any errors
    let bench_fn = || {
        let mut lexer = Lexer::new(&src);
        while lexer.next().unwrap().kind == TokenKind::EOF { /* Benchmark */ }
    };

    let avg = bench(bench_fn, 50000, 1000);

    // Calculates stats based on bench average
    let bytes_per_sec = (1_000_000_000.0 / avg.as_nanos() as f64) * src.len() as f64;
    let mb = 1000.0 * 1000.0;
    let mib = 1024.0 * 1024.0;

    println!(
        "Average lex speed: {:.3} MB/s => {:.3} MiB/s",
        bytes_per_sec / mb,
        bytes_per_sec / mib
    );
}
