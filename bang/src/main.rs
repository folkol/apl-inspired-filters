extern crate core;

use std::io::Write;

use ibig::{ubig, UBig};

use framework::{get_out, Lines, parse, print_broken_pipe, Valence};

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
        let n: usize = match parse(line) {
            Some(value) => value,
            None => continue,
        };

        let result = factorial(n);

        if print_broken_pipe(out, result) {
            break;
        }
    }
}

fn factorial(n: usize) -> UBig {
    let mut result = ubig!(1);
    for n in 1..=n {
        result *= n;
    }
    result
}

// TODO: add support for non-integers with Beta function?
fn dyadic(out: &mut Box<dyn Write>, lhs: Lines, rhs: Lines) {
    for (lhs, rhs) in lhs.zip(rhs) {
        let lhs: usize = match parse(lhs) {
            Some(value) => value,
            None => continue,
        };
        let rhs: usize = match parse(rhs) {
            Some(value) => value,
            None => continue,
        };

        let n = factorial(lhs) / (factorial(rhs) * factorial(lhs - rhs));

        if print_broken_pipe(out, n) {
            break;
        }
    }
}

