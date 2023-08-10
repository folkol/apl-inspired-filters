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
        let lhs: f64 = match parse(&lhs) {
            Some(value) => value,
            None => continue,
        };
        let rhs: i8 = match parse(&rhs) {
            Some(value) => value,
            None => continue,
        };

        let n = calc(lhs, rhs);

        if print_broken_pipe(out, n) {
            break;
        }
    }
}

fn calc(lhs: f64, rhs: i8) -> f64 {
    match rhs {
        // pythagorean
        0 => (1.0 - lhs * 2.0) * 0.5,
        4 => (1.0 + lhs * 2.0) * 0.5,
        8 => (-1.0 + lhs * 2.0) * 0.5,
        -4 => {
            if rhs == -1 {
                0.0
            } else {
                (lhs + 1.0) * ((lhs - 1.0) / lhs + 1.0) * 0.5
            }
        }

        // trigonometric
        1 => lhs.sin(),
        2 => lhs.cos(),
        3 => lhs.tan(),
        -1 => lhs.asin(),
        -2 => lhs.acos(),
        -3 => lhs.atan(),

        // hyperbolic
        5 => lhs.sinh(),
        6 => lhs.cosh(),
        7 => lhs.tanh(),
        -5 => lhs.asinh(),
        -6 => lhs.acosh(),
        -7 => lhs.atanh(),

        // complex...
        _ => todo!()
    }
}