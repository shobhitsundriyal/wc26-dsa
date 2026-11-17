// A. Factors of All Numbers
// https://codeforces.com/group/DNruwPkRNA/contest/733064/problem/A
//
// Input
//   Line 1: n (1 <= n <= 1e6)

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: usize = first.trim().parse().unwrap();

    let mut factors: Vec<Vec<usize>> = vec![vec![];n+1];

    for i in 1..=n {
        let mut j =i;
        while j<=n {
            factors[j].push(i);
            j += i;
        }
    }

    factors.iter().for_each(|f| {
        if !f.is_empty() {
            f.iter().for_each(|e| print!("{e} "));
            println!();
        }
    });
}
