extern crate core;

use std::io::Write;

use framework::{get_out, Lines, parse, print_broken_pipe, Valence};

fn main() {
    let mut out = get_out();
    match framework::read_input() {
        Valence::Niladic => {}
        Valence::Monadic(_) => {
            eprintln!("ge is dyadic");
            std::process::exit(1);
        }
        Valence::Dyadic(lhs, rhs) => dyadic(&mut out, lhs, rhs),
    }
}

fn dyadic(out: &mut Box<dyn Write>, lhs: Lines, rhs: Lines) {
    for (lhs, rhs) in lhs.zip(rhs) {
        let a: f64 = match parse(&lhs) {
            Some(value) => value,
            None => continue,
        };
        let b: f64 = match parse(&rhs) {
            Some(value) => value,
            None => continue,
        };
        print_broken_pipe(out, (a >= b) as i32);
    }
}
