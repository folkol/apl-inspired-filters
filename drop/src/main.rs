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
        let mut lhs: Vec<_> = lhs.split_whitespace().collect();
        let rhs: i32 = match parse(&rhs) {
            Some(value) => value,
            None => continue,
        };

        let lhs: Vec<_> = if rhs < 0 {
            let rhs = rhs.unsigned_abs() as usize;
            let n = lhs.len();
            lhs.into_iter().take(n.saturating_sub(rhs)).collect()
        } else {
            lhs.into_iter().skip(rhs as usize).collect()
        };

        if print_broken_pipe(out, &lhs[..].join(" ")) {
            break;
        }
    }
}
