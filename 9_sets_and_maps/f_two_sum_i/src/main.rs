// F. Two Sum - I
// https://codeforces.com/group/DNruwPkRNA/contest/729216/problem/F
//
// Input
//   Line 1: n x (1 <= n <= 1e5, -2e9 <= x <= 2e9)
//   Line 2: n integers a1 .. an (-1e9 <= ai <= 1e9)

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let x: i64 = tokens.next().unwrap().parse().unwrap();

    let a: Vec<i64> = (0..n)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();

    let _ = (&a, x);

    // TODO: solve
}
