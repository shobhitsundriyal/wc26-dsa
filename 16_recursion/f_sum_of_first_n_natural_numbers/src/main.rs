// F. Sum of first N Natural Numbers
// https://codeforces.com/group/DNruwPkRNA/contest/733421/problem/F
//
// Input
//   Line 1: n (1 <= n <= 1e5)

use std::io::{self, BufRead};

fn sum(n: usize) -> usize {
    if n == 1 {return 1;}
    
    n + sum(n-1)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: usize = first.trim().parse().unwrap();

    println!("{}",sum(n));
}
