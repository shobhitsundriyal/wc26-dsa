// G. Count Number of Distinct Integers
// https://codeforces.com/group/DNruwPkRNA/contest/698784/problem/G
//
// Input
//   Line 1: n k (1 <= k <= n <= 1e5)
//   Line 2: n integers a1 .. an (1 <= ai <= 1e9)

use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n k
    let first = lines.next().expect("missing first line").unwrap();
    let mut first = first.split_whitespace();

    let n: usize = first.next().unwrap().parse().unwrap();
    let k: usize = first.next().unwrap().parse().unwrap();

    // Line 2 (and any following lines, in case it wraps): the n array values.
    let mut a: Vec<i64> = Vec::with_capacity(n);
    while a.len() < n {
        let line = lines.next().expect("not enough array values").unwrap();
        a.extend(
            line.split_whitespace()
                .map(|t| t.parse::<i64>().unwrap()),
        );
    }

    // Frequency of each value in the current window.
    let mut freq: HashMap<i64, usize> = HashMap::new();
    for &x in &a[..k] {
        *freq.entry(x).or_insert(0) += 1;
    }
    print!("{} ", freq.len());

    for i in k..n {
        // Remove the element leaving the window.
        let out = a[i - k];
        if let Some(count) = freq.get_mut(&out) {
            *count -= 1;
            if *count == 0 {
                freq.remove(&out);
            }
        }

        // Add the element entering the window.
        *freq.entry(a[i]).or_insert(0) += 1;

        print!("{} ", freq.len());
    }
    println!();
}
