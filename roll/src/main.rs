use std::env::args;
use std::io::stdin;

use rand::prelude::*;

fn main() {
    if let Some(n) = args().nth(1) {
        roll(n);
    } else {
        for line in stdin().lines().map_while(Result::ok) {
            roll(line);
        }
    }
}

fn roll(n: String) {
    let n: usize = match n.parse() {
        Ok(n) if n == 0 => {
            eprintln!("What do you mean: \"zero sided dice\"?");
            return;
        }
        Ok(n) => n,
        Err(e) => {
            eprintln!("couldn't parse number of faces: `{e}`");
            return;
        }
    };
    let result = thread_rng().gen_range(1..=n);
    println!("{result}");
}
