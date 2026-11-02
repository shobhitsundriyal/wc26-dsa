// B. Search Queries - III
// https://codeforces.com/group/DNruwPkRNA/contest/729216/problem/B
//
// Input
//   Line 1: n q (1 <= n, q <= 1e5)
//   Line 2: n distinct integers a1 .. an (-1e9 <= ai <= 1e9)
//   Next q lines: one integer x (-1e9 <= x <= 1e9)

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let q: usize = tokens.next().unwrap().parse().unwrap();

    let a: Vec<i64> = (0..n)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();

    let queries: Vec<i64> = (0..q)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();

    let _ = (&a, &queries);

    // TODO: solve
}
