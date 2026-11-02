// M. Group Anagrams
// https://codeforces.com/group/DNruwPkRNA/contest/729216/problem/M
//
// Input
//   Line 1: n (1 <= n <= 1e5)
//   Next n lines: one string s (1 <= |s| <= 100), lowercase English letters

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();

    let strings: Vec<String> = (0..n)
        .map(|_| tokens.next().unwrap().to_string())
        .collect();

    let _ = &strings;

    // TODO: solve
}
