// G. Sum of All Subarrays (Hard)
// https://codeforces.com/group/DNruwPkRNA/contest/726083/problem/G
//
// Input
//   Line 1: n (1 <= n <= 10^5)
//   Line 2: n integers a1, a2, ..., an (|ai| <= 10^9)
//
// Output
//   A single integer: the sum of the sums of all subarrays.
//   Example: [1, 2, 3] -> 20
//
// Note: the answer can get very large, so use a wide integer type (e.g. i128).
// With n up to 10^5, an O(n^2) solution will be too slow -- aim for O(n).

use std::io::{self, BufRead};

fn main() {
    /// ------------------------------------------------------------------
    // Input boilerplate (you normally don't need to change this part)
    // ------------------------------------------------------------------
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let n= lines
        .next()
        .expect("missing first line")
        .expect("failed to read input")
        .trim()
        .parse()
        .expect("n must be an integer");

    // Line 2 (and any following lines, in case it wraps): the n array values.
    // Reading stops as soon as n values are read -- no Ctrl+D needed.
    let mut a: Vec<i64> = Vec::with_capacity(n);
    while a.len() < n {
        let line = lines
            .next()
            .expect("not enough array values")
            .expect("failed to read input");
        a.extend(
            line.split_whitespace()
                .map(|t| t.parse::<i64>().expect("array values must be integers")),
        );
    }

    // optimizist O(n)
    let mut i: usize = 0;
    let mut final_sum: i64=0;
    while i<n {
        final_sum += a[i] * ((n-i) as i64) * ((i+1) as i64);
        i+=1;
    }
    println!("{final_sum}");
}
