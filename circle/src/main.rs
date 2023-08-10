extern crate core;

use std::io::Write;

use framework::{get_out, Lines, parse, print_broken_pipe, Valence};

fn main() {
    let mut out = get_out();
    match framework::read_input() {
        Valence::Niladic => {}
        Valence::Monadic(lhs) => monadic(&mut out, lhs),
        Valence::Dyadic(lhs, rhs) => dyadic(&mut out, lhs, rhs),
    }
}

fn monadic(out: &mut Box<dyn Write>, lhs: Lines) {
    for n in lhs {
        let n: f64 = match parse(&n) {
            Some(value) => value,
            None => continue,
        };

        let n = n * std::f64::consts::PI;
        if print_broken_pipe(out, n) {
            break;
        }
    }
}

fn dyadic(out: &mut Box<dyn Write>, lhs: Lines, rhs: Lines) {
    for (lhs, rhs) in lhs.zip(rhs) {
        let lhs: usize = match parse(&lhs) {
            Some(value) => value,
            None => continue,
        };
        let rhs: usize = match parse(&rhs) {
            Some(value) => value,
            None => continue,
        };

        let n = lcd(lhs, rhs);
        if print_broken_pipe(out, n) {
            break;
        }
    }
}

fn lcd(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}