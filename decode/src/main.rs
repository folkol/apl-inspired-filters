extern crate core;

use std::io::Write;

use framework::{get_out, Lines, parse, print_broken_pipe, Valence};

fn main() {
    let mut out = get_out();
    match framework::read_input() {
        Valence::Niladic => {}
        Valence::Monadic(_) => {
            eprintln!("decode is dyadic");
            std::process::exit(1);
        }
        Valence::Dyadic(lhs, rhs) => dyadic(&mut out, lhs, rhs),
    }
}

fn dyadic(out: &mut Box<dyn Write>, lhs: Lines, rhs: Lines) {
    for (lhs, rhs) in lhs.zip(rhs) {
        let digits_maybe: Vec<_> = lhs.split_whitespace().map(parse::<usize>).collect();
        if digits_maybe.iter().any(|n| n.is_none()) {
            continue;
        }
        let digits = digits_maybe;
        let base: usize = match parse(&rhs) {
            Some(value) => value,
            None => continue,
        };
        if base < 1 {
            eprintln!("Base must be 1+");
            std::process::exit(1);
        }
        if digits.iter().any(|n| {
            let n = n.unwrap();
            let out_of_bounds = n >= base;
            if out_of_bounds {
                eprintln!("coefficient `{n}` larger than base `{base}`");
            }
            out_of_bounds
        }) {
            continue;
        }

        let mut sum = 0;
        for (i, digit) in digits.iter().rev().enumerate() {
            let digit: usize = digit.unwrap();
            sum += digit * base.pow(i as u32);
        }

        if print_broken_pipe(out, sum) {
            break;
        }
    }
}
