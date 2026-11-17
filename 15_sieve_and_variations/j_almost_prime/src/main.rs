// J. Almost Prime
// https://codeforces.com/group/DNruwPkRNA/contest/733064/problem/J
//
// Input
//   Line 1: n (1 <= n <= 5 * 1e6)

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: usize = first.trim().parse().unwrap();

    let _ = n;

    // TODO: solve
}
