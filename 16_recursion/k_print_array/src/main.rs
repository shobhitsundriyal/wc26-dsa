// K. Print Array
// https://codeforces.com/group/DNruwPkRNA/contest/733421/problem/K
//
// Input
//   Line 1: n (1 <= n <= 1e5)
//   Line 2: n integers a1 .. an (-1e9 <= ai <= 1e9)

use std::io::{self, BufRead};

fn print_arr(arr: &[i64], i:usize, n: usize) {
    if i==n {return}
    print!("{} ", arr[i]);
    print_arr(arr, i+1, n);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: usize = first.trim().parse().unwrap();

    // Line 2 (and any following lines, in case it wraps): the n values.
    let mut a: Vec<i64> = Vec::with_capacity(n);
    while a.len() < n {
        let line = lines.next().expect("not enough array values").unwrap();
        a.extend(
            line.split_whitespace()
                .map(|t| t.parse::<i64>().unwrap()),
        );
    }

    print_arr(&a, 0, n);
}
