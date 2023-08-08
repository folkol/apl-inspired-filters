use std::io::{BufWriter, stdout, Write};

use atty::Stream::Stdout;

use framework::{Lines, parse, print_broken_pipe, Valence};

fn main() {
    let mut out: Box<dyn Write> = if atty::is(Stdout) {
        Box::new(stdout().lock())
    } else {
        Box::new(BufWriter::new(stdout().lock()))
    };
    match framework::read_input() {
        Valence::Niladic => {}
        Valence::Monadic(lines) => monadic(&mut out, lines),
        Valence::Dyadic(lhs, rhs) => dyadic(&mut out, lhs, rhs),
    }
}

fn monadic(out: &mut Box<dyn Write>, lines: Lines) {
    for line in lines {
        let n: f64 = match parse(line) {
            Some(value) => value,
            None => continue,
        };
        let n = n.ceil() as i64;
        if print_broken_pipe(out, n) {
            break;
        }
    }
}

fn dyadic(out: &mut Box<dyn Write>, lhs: Lines, rhs: Lines) {
    for (lhs, rhs) in lhs.zip(rhs) {
        let lhs: f64 = match parse(lhs) {
            Some(value) => value,
            None => continue,
        };
        let rhs: f64 = match parse(rhs) {
            Some(value) => value,
            None => continue,
        };

        let n = lhs.max(rhs);
        if print_broken_pipe(out, n) {
            break;
        }
    }
}

