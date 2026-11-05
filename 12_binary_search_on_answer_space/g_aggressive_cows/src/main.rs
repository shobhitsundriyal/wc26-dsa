// G. Aggressive Cows
// https://codeforces.com/group/DNruwPkRNA/contest/730116/problem/G
//
// Input
//   Line 1: n k (2 <= k <= n <= 2e5)
//   Line 2: n integers x1 .. xn (1 <= xi <= 1e12)

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n k
    let first = lines.next().expect("missing first line").unwrap();
    let mut first = first.split_whitespace();

    let n: usize = first.next().unwrap().parse().unwrap();
    let k: usize = first.next().unwrap().parse().unwrap();

    // Line 2 (and any following lines, in case it wraps): the n stall positions.
    let mut x: Vec<u64> = Vec::with_capacity(n);
    while x.len() < n {
        let line = lines.next().expect("not enough positions").unwrap();
        x.extend(
            line.split_whitespace()
                .map(|t| t.parse::<u64>().unwrap()),
        );
    }

    let _ = (&x, k);

    // TODO: solve
}
