#![feature(slice_concat_trait)]
extern crate core;

use std::io::Write;

use rand::prelude::*;

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
        if roll(out, line) { break; };
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

        let mut rng = thread_rng();
        let mut vec: Vec<_> = (1..=rhs).map(|n| n.to_string()).collect();
        vec.shuffle(&mut rng);

        if lhs >= vec.len() {
            eprintln!("Can't deal {lhs} out of {} items.", vec.len());
            continue;
        }
        let deal = &vec[..lhs];

        if print_broken_pipe(out, deal.join(" ")) {
            break;
        }
    }
}

fn roll(out: &mut Box<dyn Write>, n: String) -> bool {
    let n: usize = match n.parse() {
        Ok(n) if n == 0 => {
            eprintln!("What do you mean: \"zero sided dice\"?");
            return false;
        }
        Ok(n) => n,
        Err(e) => {
            eprintln!("couldn't parse number of faces: `{e}`");
            return false;
        }
    };
    let result = thread_rng().gen_range(1..=n);
    print_broken_pipe(out, format!("{result}"))
}
