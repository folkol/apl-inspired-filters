extern crate core;

use std::io::Write;

use framework::{get_out, Lines, print_broken_pipe, Valence};

fn main() {
    let mut out = get_out();
    match framework::read_input() {
        Valence::Niladic => {}
        Valence::Monadic(_) => {
            eprintln!("equal is dyadic");
            std::process::exit(1);
        }
        Valence::Dyadic(lhs, rhs) => dyadic(&mut out, lhs, rhs),
    }
}

fn dyadic(out: &mut Box<dyn Write>, lhs: Lines, rhs: Lines) {
    for (lhs, rhs) in lhs.zip(rhs) {
        let result = is_equal(&lhs, &rhs);
        print_broken_pipe(out, result);
    }
}

fn is_equal(lhs: &String, rhs: &String) -> bool {
    if lhs == rhs {
        return true;
    }

    let a: Result<usize, _> = lhs.parse();
    let b: Result<usize, _> = rhs.parse();
    if let (Ok(a), Ok(b)) = (a, b) {
        return a == b;
    }

    let a: Result<f64, _> = lhs.parse();
    let b: Result<f64, _> = rhs.parse();
    if let (Ok(a), Ok(b)) = (a, b) {
        return (b - a).abs() < f64::MIN_POSITIVE;
    }

    false
}