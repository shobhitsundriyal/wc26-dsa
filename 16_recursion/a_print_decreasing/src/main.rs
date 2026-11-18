// A. Print Decreasing
// https://codeforces.com/group/DNruwPkRNA/contest/733421/problem/A
//
// Input
//   Line 1: n (1 <= n <= 1e4)

use std::io::{self, BufRead}; 
fn print(n: usize) {
    if n == 0 {return;}
    println!("{n}");
    print(n-1);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: usize = first.trim().parse().unwrap();

    print(n);
}
