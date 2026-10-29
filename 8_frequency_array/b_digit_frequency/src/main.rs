// B. Digit Frequency
// https://codeforces.com/group/DNruwPkRNA/contest/728252/problem/B
//
// Input
//   Line 1: N (0 <= N <= 10^100000), given without leading zeros
//           (except N = 0), so read it as a string.

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: String = tokens.next().unwrap().to_string();

    let _ = &n;

    // TODO: solve
}
