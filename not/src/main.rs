#![feature(let_chains)]
extern crate core;

use std::io::Write;

use framework::{get_out, Lines, print_broken_pipe, Valence};

fn main() {
    let mut out = get_out();
    match framework::read_input() {
        Valence::Niladic => {}
        Valence::Monadic(lhs) => {
            monadic(&mut out, lhs);
        }
        Valence::Dyadic(lhs, rhs) => dyadic(&mut out, lhs, rhs),
    }
}

fn monadic(out: &mut Box<dyn Write>, lhs: Lines) {
    for line in lhs {
        if let Ok(b) = line.parse::<bool>() {
            print_broken_pipe(out, !b);
        } else if let Ok(b) = line.parse::<u8>() && b == 0 {
            print_broken_pipe(out, 1);
        } else if let Ok(b) = line.parse::<u8>() && b == 1 {
            print_broken_pipe(out, 0);
        } else {
            eprintln!("expected 0, 1 or boolean");
        }
    }
}

fn dyadic(out: &mut Box<dyn Write>, lhs: Lines, rhs: Lines) {
    for (a, b) in lhs.zip(rhs) {
        let filtered_chars: String = a.chars().filter(|c| !b.chars().any(|d| *c == d)).collect();
        print_broken_pipe(out, filtered_chars);
    }
}