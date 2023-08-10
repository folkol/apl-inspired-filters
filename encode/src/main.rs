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
        let lhs: usize = match parse(&lhs) {
            Some(value) => value,
            None => continue,
        };
        let base: usize = match parse(&rhs) {
            Some(value) => value,
            None => continue,
        };
        if base < 1 {
            eprintln!("Base must be 1+");
            std::process::exit(1);
        }

        let mut digits: Vec<usize> = Vec::new();
        let msb = lhs.ilog(base);
        let mut residue = lhs;
        for pos in (0..=msb).rev() {
            let digit = residue / base.pow(pos);
            digits.push(digit);
            residue -= digit * base.pow(pos)
        }

        let digits: Vec<_> = digits.iter().map(&usize::to_string).collect();
        if print_broken_pipe(out, &digits[..].join(" ")) {
            break;
        }
    }
}
