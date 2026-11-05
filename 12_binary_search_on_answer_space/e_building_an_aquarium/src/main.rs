// E. Building an Aquarium
// https://codeforces.com/group/DNruwPkRNA/contest/730116/problem/E
//
// Input
//   Line 1: t (1 <= t <= 1e4)
//   For each test case:
//     Line 1: n x (1 <= n <= 2e5, 1 <= x <= 1e9)
//     Line 2: n integers a1 .. an (1 <= ai <= 1e9)
//   Sum of n over all test cases does not exceed 2e5.

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: t
    let first = lines.next().expect("missing first line").unwrap();
    let t: usize = first.trim().parse().unwrap();

    let mut test_cases: Vec<(usize, u64, Vec<u64>)> = Vec::with_capacity(t);

    for _ in 0..t {
        // Line 1 of this test case: n x
        let header = lines.next().expect("missing test case header").unwrap();
        let mut header = header.split_whitespace();

        let n: usize = header.next().unwrap().parse().unwrap();
        let x: u64 = header.next().unwrap().parse().unwrap();

        // Line 2 (and any following lines, in case it wraps): the n heights.
        let mut a: Vec<u64> = Vec::with_capacity(n);
        while a.len() < n {
            let line = lines.next().expect("not enough heights").unwrap();
            a.extend(
                line.split_whitespace()
                    .map(|v| v.parse::<u64>().unwrap()),
            );
        }

        test_cases.push((n, x, a));
    }

    let _ = &test_cases;

    // TODO: solve
}
