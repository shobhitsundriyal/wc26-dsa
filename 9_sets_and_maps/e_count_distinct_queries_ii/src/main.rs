// E. Count Distinct Queries - II
// https://codeforces.com/group/DNruwPkRNA/contest/729216/problem/E
//
// Input
//   Line 1: q (1 <= q <= 2 * 1e5)
//   Next q lines: one of
//     "1 x"  insert x
//     "2 x"  remove one occurrence of x (if present)
//     "3"    print the number of distinct elements
//     "4 x"  print YES if x is present, otherwise NO
//   (-1e9 <= x <= 1e9)

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let q: usize = tokens.next().unwrap().parse().unwrap();

    for _ in 0..q {
        let kind: u8 = tokens.next().unwrap().parse().unwrap();

        match kind {
            1 | 2 | 4 => {
                let x: i64 = tokens.next().unwrap().parse().unwrap();
                let _ = x;

                // TODO: solve
            }
            3 => {
                // TODO: print the number of distinct elements
            }
            _ => unreachable!("unknown query type: {kind}"),
        }
    }
}
