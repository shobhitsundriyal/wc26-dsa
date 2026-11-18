// S. Tower Of Hanoi
// https://codeforces.com/group/DNruwPkRNA/contest/733421/problem/S
//
// Input
//   Line 1: n (1 <= n <= 20)

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
