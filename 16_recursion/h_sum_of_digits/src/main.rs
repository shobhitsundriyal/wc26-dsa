// H. Sum Of Digits
// https://codeforces.com/group/DNruwPkRNA/contest/733421/problem/H
//
// Input
//   Line 1: n (0 <= n <= 1e18)

use std::io::{self, BufRead};

fn sum_digit (n: u64) -> u64 {
    if n == 0 {return 0;}

    n%10 + sum_digit(n/10)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: u64 = first.trim().parse().unwrap();

    println!("{}", sum_digit(n));
}
