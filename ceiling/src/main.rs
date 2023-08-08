use std::io::{BufWriter, stdout, Write};
use std::io::ErrorKind::BrokenPipe;

use atty::Stream::Stdout;

use framework::{Arity, Lines};

fn main() {
    let mut out: Box<dyn Write> = if atty::is(Stdout) {
        Box::new(stdout().lock())
    } else {
        Box::new(BufWriter::new(stdout().lock()))
    };
    let arity = framework::read_input();
    if let Arity::Monadic(lines) = arity {
        monadic(&mut out, lines);
    }
    println!("done");
}

fn monadic(out: &mut Box<dyn Write>, lines: Lines) {
    for line in lines {
        let n = match line.parse::<f64>() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Couldn't parse number: `{line}` ({e})");
                continue;
            }
        };
        let n = n.ceil() as i64;
        match writeln!(out, "{n}") {
            Ok(_) => {}
            Err(e) if e.kind() == BrokenPipe => { break; }
            Err(e) => {
                eprintln!("Failed writing output: {e}");
                std::process::exit(1);
            }
        }
    }
}
