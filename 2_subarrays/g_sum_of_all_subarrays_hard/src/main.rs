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
