use std::io::Write;

use framework::{get_out, Lines, parse, print_broken_pipe, Valence};

/// downstile: floor / minimum
fn main() {

    let mut out = get_out();
    match framework::read_input() {
        Valence::Niladic => {}
        Valence::Monadic(lines) => monadic(&mut out, lines),
        Valence::Dyadic(lhs, rhs) => dyadic(&mut out, lhs, rhs),
    }
}

fn monadic(out: &mut Box<dyn Write>, lines: Lines) {
    for line in lines {
        let n: f64 = match parse(&line) {
            Some(value) => value,
            None => continue,
        };
        let n = n.floor() as i64;
        if print_broken_pipe(out, n) {
            break;
        }
    }
}

fn dyadic(out: &mut Box<dyn Write>, lhs: Lines, rhs: Lines) {
    for (lhs, rhs) in lhs.zip(rhs) {
        let lhs: f64 = match parse(&lhs) {
            Some(value) => value,
            None => continue,
        };
        let rhs: f64 = match parse(&rhs) {
            Some(value) => value,
            None => continue,
        };

        let n = lhs.min(rhs);
        if print_broken_pipe(out, n) {
            break;
        }
    }
}

