use std::env::args;
use std::fmt::{Debug, Display, Formatter};
use std::io::{stdin, Write};
use std::io::ErrorKind::BrokenPipe;
use std::iter::repeat;
use std::str::FromStr;

pub type Lines = Box<dyn Iterator<Item=String>>;

pub enum Valence {
    Niladic,
    Monadic(Lines),
    Dyadic(Lines, Lines),
}

impl Debug for Valence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let dbg = match &self {
            Valence::Niladic => "Niladic",
            Valence::Monadic(_) => "Monadic",
            Valence::Dyadic(_, _) => "Dyadic"
        };
        f.write_str(dbg)
    }
}

pub fn read_input() -> Valence {
    let args: Vec<_> = args().skip(1).collect();
    let mut lines = stdin().lines().map_while(Result::ok).peekable();
    if args.is_empty() {
        match lines.peek() {
            None => Valence::Niladic,
            Some(_) => {
                Valence::Monadic(Box::new(lines))
            }
        }
    } else {
        match lines.peek() {
            None => Valence::Monadic(Box::new(lines)),
            Some(_) => {
                let num_args = args.len();
                if num_args == 1 {
                    if let Some(scalar) = args.into_iter().next() {
                        Valence::Dyadic(Box::new(lines), Box::new(repeat(scalar)))
                    } else {
                        Valence::Niladic
                    }
                } else {
                    Valence::Dyadic(Box::new(lines), Box::new(args.into_iter()))
                }
            }
        }
    }
}

pub fn print_broken_pipe<T>(out: &mut Box<dyn Write>, n: T) -> bool
    where
        T: Display,
{
    match writeln!(out, "{n}") {
        Ok(_) => {}
        Err(e) if e.kind() == BrokenPipe => {
            return true;
        }
        Err(e) => {
            eprintln!("Failed writing output: {e}");
            std::process::exit(1);
        }
    }
    false
}

pub fn parse<T>(lhs: String) -> Option<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Display,
{
    let lhs = match lhs.parse::<T>() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Couldn't parse number: `{lhs}` ({e})");
            return None;
        }
    };
    Some(lhs)
}


// pub fn write_output(lines: Lines) -> ! {
//     let mut out = BufWriter::new(stdout().lock());
//     for line in lines {
//         match write!(out, "{line}") {
//             Ok(_) => {}
//             Err(e) if e.kind() == BrokenPipe => { break; }
//             Err(e) => {
//                 eprintln!("Failed writing output: {e}");
//                 std::process::exit(1);
//             }
//         }
//     }
//     std::process::exit(0);
// }
