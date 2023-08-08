use std::env::args;
use std::fmt::{Debug, Formatter};
use std::io::stdin;

pub type Lines = Box<dyn Iterator<Item=String>>;

pub enum Arity {
    Niladic,
    Monadic(Lines),
    Dyadic,
}

impl Debug for Arity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let dbg = match &self {
            Arity::Niladic => "Niladic",
            Arity::Monadic(_) => "Monadic",
            Arity::Dyadic => "Dyadic"
        };
        f.write_str(dbg)
    }
}


// TODO: Extend arguments to match rank?
pub fn read_input() -> Arity {
    let args: Vec<_> = args().skip(1).collect();
    if args.is_empty() {
        return Arity::Monadic(Box::new(stdin().lines().map_while(Result::ok)));
    }
    Arity::Niladic
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
