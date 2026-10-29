// K. Atleast Twice Queries
// https://codeforces.com/group/DNruwPkRNA/contest/728252/problem/K
//
// Input
//   Line 1: q (1 <= q <= 1e6)
//   Next q lines: one of
//     "1 x"  add x
//     "2 x"  remove one occurrence of x (if present)
//     "3"    print how many distinct numbers appear at least twice
//   (1 <= x <= 1e5)

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let q: usize = tokens.next().unwrap().parse().unwrap();

    for _ in 0..q {
        let kind: u8 = tokens.next().unwrap().parse().unwrap();

        if kind == 3 {
            // TODO: print the number of distinct values occurring at least twice
        } else {
            let x: usize = tokens.next().unwrap().parse().unwrap();
            let _ = x;

            // TODO: add / remove x
        }
    }
}
