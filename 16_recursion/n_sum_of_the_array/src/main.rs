// N. Sum of the Array
// https://codeforces.com/group/DNruwPkRNA/contest/733421/problem/N
//
// Input
//   Line 1: n (1 <= n <= 1e5)
//   Line 2: n integers a1 .. an (-1e9 <= ai <= 1e9)

use std::io::{self, BufRead};

fn sum_arr(a: &[i64], n: usize) -> i64 {
    if n==0 {
        return  0;
    }
    a[n] + sum_arr(a, n-1)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: usize = first.trim().parse().unwrap();

    // Line 2 (and any following lines, in case it wraps): the n values.
    let mut a: Vec<i64> = Vec::with_capacity(n);
    a.push(0);
    while a.len() < n+1 {
        let line = lines.next().expect("not enough array values").unwrap();
        a.extend(
            line.split_whitespace()
                .map(|t| t.parse::<i64>().unwrap()),
        );
    }

    println!("{}",sum_arr(&a, n));
}
