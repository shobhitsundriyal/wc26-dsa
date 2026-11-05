// B. Ship Packages Within D Days
// https://codeforces.com/group/DNruwPkRNA/contest/730116/problem/B
//
// Input
//   Line 1: n d (1 <= n <= 2e5, 1 <= d <= n)
//   Line 2: n integers w1 .. wn (1 <= wi <= 1e9)

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n d
    let first = lines.next().expect("missing first line").unwrap();
    let mut first = first.split_whitespace();

    let n: usize = first.next().unwrap().parse().unwrap();
    let d: usize = first.next().unwrap().parse().unwrap();

    // Line 2 (and any following lines, in case it wraps): the n weights.
    let mut w: Vec<u64> = Vec::with_capacity(n);
    while w.len() < n {
        let line = lines.next().expect("not enough weights").unwrap();
        w.extend(
            line.split_whitespace()
                .map(|t| t.parse::<u64>().unwrap()),
        );
    }

    let _ = (&w, d);

    // TODO: solve
}
