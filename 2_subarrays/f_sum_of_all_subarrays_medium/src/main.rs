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

use std::io::{self, BufRead};

fn main() {
    /// ------------------------------------------------------------------
    // Input boilerplate (you normally don't need to change this part)
    // ------------------------------------------------------------------
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let n: usize = lines
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

    // optimized n^2
    let mut i = 0;
    let mut final_sum=0;
    while i<=n {
        let mut j = i;
        let mut sub_array_sum = 0;
        while j<n {
            sub_array_sum += a[j];
            final_sum += sub_array_sum;
            j+=1;
        }    
        // println!("{sub_array_sum}");
        i+=1;
    }
    println!("{final_sum}");
}
