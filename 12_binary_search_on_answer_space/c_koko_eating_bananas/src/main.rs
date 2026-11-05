// C. Koko Eating Bananas
// https://codeforces.com/group/DNruwPkRNA/contest/730116/problem/C
//
// Input
//   Line 1: n h (1 <= n <= 2e5, 1 <= h <= 1e18)
//   Line 2: n integers a1 .. an (1 <= ai <= 1e9)

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n h
    let first = lines.next().expect("missing first line").unwrap();
    let mut first = first.split_whitespace();

    let n: usize = first.next().unwrap().parse().unwrap();
    let h: u64 = first.next().unwrap().parse().unwrap();

    // Line 2 (and any following lines, in case it wraps): the n piles.
    let mut a: Vec<u64> = Vec::with_capacity(n);
    while a.len() < n {
        let line = lines.next().expect("not enough piles").unwrap();
        a.extend(
            line.split_whitespace()
                .map(|t| t.parse::<u64>().unwrap()),
        );
    }

    let _ = (&a, h);

    // TODO: solve
}
