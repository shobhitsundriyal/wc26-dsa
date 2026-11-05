// A. Integer Square Root
// https://codeforces.com/group/DNruwPkRNA/contest/730116/problem/A
//
// Input
//   Line 1: n (0 <= n <= 1e18)

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: u64 = first.trim().parse().unwrap();

    let _ = n;

    // TODO: solve
}
