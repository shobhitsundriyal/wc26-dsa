// F. Allocate Minimum Pages
// https://codeforces.com/group/DNruwPkRNA/contest/730116/problem/F
//
// Input
//   Line 1: n k (1 <= n <= 2e5, 1 <= k <= 2e5)
//   Line 2: n integers a1 .. an (1 <= ai <= 1e9)

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n k
    let first = lines.next().expect("missing first line").unwrap();
    let mut first = first.split_whitespace();

    let n: usize = first.next().unwrap().parse().unwrap();
    let k: usize = first.next().unwrap().parse().unwrap();

    // Line 2 (and any following lines, in case it wraps): the n page counts.
    let mut a: Vec<u64> = Vec::with_capacity(n);
    while a.len() < n {
        let line = lines.next().expect("not enough page counts").unwrap();
        a.extend(
            line.split_whitespace()
                .map(|t| t.parse::<u64>().unwrap()),
        );
    }

    let _ = (&a, k);

    // TODO: solve
}
