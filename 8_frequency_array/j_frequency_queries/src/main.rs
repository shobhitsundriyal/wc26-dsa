// J. Frequency Queries
// https://codeforces.com/group/DNruwPkRNA/contest/728252/problem/J
//
// Input
//   Line 1: q (1 <= q <= 1e6)
//   Next q lines: one of
//     "1 x"    add x
//     "2 x"    remove one occurrence of x (if present)
//     "3 a b"  print YES if count(a) == count(b), otherwise NO
//   (1 <= x, a, b <= 1e5)

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let q: usize = tokens.next().unwrap().parse().unwrap();

    for _ in 0..q {
        let kind: u8 = tokens.next().unwrap().parse().unwrap();

        match kind {
            1 | 2 => {
                let x: usize = tokens.next().unwrap().parse().unwrap();
                let _ = x;

                // TODO: add / remove x
            }
            3 => {
                let a: usize = tokens.next().unwrap().parse().unwrap();
                let b: usize = tokens.next().unwrap().parse().unwrap();
                let _ = (a, b);

                // TODO: print YES if count(a) == count(b), otherwise NO
            }
            _ => unreachable!("unknown query type: {kind}"),
        }
    }
}
