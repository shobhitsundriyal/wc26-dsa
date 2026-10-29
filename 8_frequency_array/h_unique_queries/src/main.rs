// H. Unique Queries
// https://codeforces.com/group/DNruwPkRNA/contest/728252/problem/H
//
// Input
//   Line 1: q (1 <= q <= 1e6)
//   Next q lines: one of
//     "+ x"  insert x
//     "- x"  remove one occurrence of x (if present)
//     "?"    count unique elements
//   (1 <= x <= 1e5)

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let q: usize = tokens.next().unwrap().parse().unwrap();

    for _ in 0..q {
        let op = tokens.next().unwrap();

        match op {
            "?" => {
                // TODO: print the number of unique elements
            }
            "+" | "-" => {
                let x: usize = tokens.next().unwrap().parse().unwrap();
                let _ = x;

                // TODO: insert / remove x
            }
            _ => unreachable!("unknown query type: {op}"),
        }
    }
}
