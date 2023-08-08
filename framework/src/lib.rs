use std::env::args;
use std::fmt::{Debug, Formatter};
use std::io::stdin;

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

// TODO: Extend arguments to match rank?
pub fn read_input() -> Valence {
    let args: Vec<_> = args().skip(1).collect();
    if args.is_empty() {
        let lines = stdin().lines().map_while(Result::ok);
        Valence::Monadic(Box::new(lines))
    } else {
        let lhs = stdin().lines().map_while(Result::ok);
        let rhs = args.into_iter();
        Valence::Dyadic(Box::new(lhs), Box::new(rhs))
    }
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
