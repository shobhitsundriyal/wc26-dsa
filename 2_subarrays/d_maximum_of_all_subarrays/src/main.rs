// D. Maximum of All Subarrays
// https://codeforces.com/group/DNruwPkRNA/contest/726083/problem/D
//
// Input
//   Line 1: n (1 <= n <= 3000)
//   Line 2: n integers a1, a2, ..., an (0 <= ai <= 10^6)
//
// Output
//   For every subarray in "L to R" order, print its maximum on its own line.
//   Example: [1, 2, 3] -> 1, 2, 3, 2, 3, 3

use std::io::{self, BufRead, Read};

fn main() {
    // ------------------------------------------------------------------
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

    let mut i = 0;
    while i<n {
        let mut j = i;
        let mut max = a[i];
        while j<n {
            max = if a[j]>max {
                a[j]
            } else {
                max
            };
            println!("{max}");
            j+=1;
        }    
        i+=1;
    }
}
