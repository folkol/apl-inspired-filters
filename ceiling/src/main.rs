use std::io::{BufWriter, stdout, Write};
use std::io::ErrorKind::BrokenPipe;

use atty::Stream::Stdout;

use framework::{Lines, Valence};

fn main() {
    let mut out: Box<dyn Write> = if atty::is(Stdout) {
        Box::new(stdout().lock())
    } else {
        Box::new(BufWriter::new(stdout().lock()))
    };
    match framework::read_input() {
        Valence::Niladic => {}
        Valence::Monadic(lines) => monadic(&mut out, lines),
        Valence::Dyadic(lhs, rhs) => dyadic(&mut out, lhs, rhs)
    }
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

fn dyadic(out: &mut Box<dyn Write>, lhs: Lines, rhs: Lines) {
    for (lhs, rhs) in lhs.zip(rhs) {
        let lhs = match parse(lhs) {
            Some(value) => value,
            None => continue,
        };
        let rhs = match parse(rhs) {
            Some(value) => value,
            None => continue,
        };

        let n = lhs.max(rhs);
        if print_pipe_softfail(out, n) { break; }
    }
}

fn print_pipe_softfail(out: &mut Box<dyn Write>, n: f64) -> bool {
    match writeln!(out, "{n}") {
        Ok(_) => {}
        Err(e) if e.kind() == BrokenPipe => { return true; }
        Err(e) => {
            eprintln!("Failed writing output: {e}");
            std::process::exit(1);
        }
    }
    false
}

fn parse(lhs: String) -> Option<f64> {
    let lhs = match lhs.parse::<f64>() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Couldn't parse number: `{lhs}` ({e})");
            return None;
        }
    };
    Some(lhs)
}
