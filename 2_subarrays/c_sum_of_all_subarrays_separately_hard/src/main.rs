// C. Sum of All Subarrays Separately (Hard)
// https://codeforces.com/group/DNruwPkRNA/contest/726083/problem/C
//
// Input
//   Line 1: n (1 <= n <= 3000)
//   Line 2: n integers a1, a2, ..., an (1 <= ai <= 10^6)
//
// Output
//   For every subarray in "L to R" order, print its sum on its own line.
//   Example: [1, 2, 3] -> 1, 3, 6, 2, 5, 3
//
// (Same as B, but n is larger -- think about the time complexity.)

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
