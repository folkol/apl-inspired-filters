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
    for line in lhs {
        let a: f64 = match parse(&line) {
            Some(value) => value,
            None => continue,
        };
        print_broken_pipe(out, a.abs());
    }
}

fn dyadic(out: &mut Box<dyn Write>, lhs: Lines, rhs: Lines) {
    for (lhs, rhs) in lhs.zip(rhs) {
        let a: usize = match parse(&lhs) {
            Some(value) => value,
            None => continue,
        };
        let b: usize = match parse(&rhs) {
            Some(value) => value,
            None => continue,
        };
        if b == 0 {
            eprintln!("Can't take {a} mod 0");
            continue;
        }
        print_broken_pipe(out, a % b);
    }
}
