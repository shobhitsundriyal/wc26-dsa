// F. Sum of All Subarrays (Medium)
// https://codeforces.com/group/DNruwPkRNA/contest/726083/problem/F
//
// Input
//   Line 1: n (1 <= n <= 5000)
//   Line 2: n integers a1, a2, ..., an (|ai| <= 10^9)
//
// Output
//   A single integer: the sum of the sums of all subarrays.
//   Example: [1, 2, 3] -> 20
//
// Note: the answer can get large, so use a wide integer type (e.g. i128).

use std::io::{self, Read};

fn main() {
    // ------------------------------------------------------------------
    // Input boilerplate (you normally don't need to change this part)
    // ------------------------------------------------------------------
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read stdin");

    let mut tokens = input.split_whitespace();

    let n: usize = tokens
        .next()
        .expect("missing n")
        .parse()
        .expect("n must be an integer");

    let a: Vec<i64> = tokens
        .take(n)
        .map(|t| t.parse().expect("array values must be integers"))
        .collect();

    // ------------------------------------------------------------------
    // TODO: write your solution here.
    //
    // `n` = number of elements
    // `a` = the array, 0-indexed (a[0] is a1)
    // Use print! / println! to produce the output.
    // ------------------------------------------------------------------

    let _ = (n, &a); // remove this line once you start using the inputs
}
