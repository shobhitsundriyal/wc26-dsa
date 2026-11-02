// J. Union of Two Arrays
// https://codeforces.com/group/DNruwPkRNA/contest/729216/problem/J
//
// Input
//   Line 1: n (1 <= n <= 1e5)
//   Line 2: n integers a1 .. an (-1e9 <= ai <= 1e9)
//   Line 3: m (1 <= m <= 1e5)
//   Line 4: m integers b1 .. bm (-1e9 <= bi <= 1e9)

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();

    let a: Vec<i64> = (0..n)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();

    let m: usize = tokens.next().unwrap().parse().unwrap();

    let b: Vec<i64> = (0..m)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();

    let _ = (&a, &b);

    // TODO: solve
}
