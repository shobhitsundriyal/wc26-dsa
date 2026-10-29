// F. Count Occurences
// https://codeforces.com/group/DNruwPkRNA/contest/728252/problem/F
//
// Input
//   Line 1: n (1 <= n <= 1e6)
//   Line 2: n integers a1 .. an (1 <= ai <= 1000)

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();

    let a: Vec<usize> = (0..n)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();

    let _ = &a;

    // TODO: solve
}
