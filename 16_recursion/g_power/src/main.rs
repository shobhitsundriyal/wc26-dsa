// G. Power
// https://codeforces.com/group/DNruwPkRNA/contest/733421/problem/G
//
// Input
//   Line 1: x n (1 <= x <= 10, 1 <= n <= 18)

use std::io::{self, BufRead};

fn power(x:u64, n:u64) -> u64 {
    if n == 1 {
        return x;
    }

    x * power(x, n-1)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: x n
    let first = lines.next().expect("missing first line").unwrap();
    let mut first = first.split_whitespace();

    let x: u64 = first.next().unwrap().parse().unwrap();
    let n: u64 = first.next().unwrap().parse().unwrap();

    println!("{}",power(x, n));
}
