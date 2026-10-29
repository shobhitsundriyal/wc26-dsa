// G. First Non-Repeating Character
// https://codeforces.com/group/DNruwPkRNA/contest/728252/problem/G
//
// Input
//   Line 1: s (1 <= |s| <= 1e5), lowercase English letters

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let s: String = tokens.next().unwrap().to_string();

    let _ = &s;

    // TODO: solve
}
