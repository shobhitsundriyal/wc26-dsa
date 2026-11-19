// R. Indices of X
// https://codeforces.com/group/DNruwPkRNA/contest/733421/problem/R
//
// Input
//   Line 1: n (1 <= n <= 1e5)
//   Line 2: n integers a1 .. an (-1e9 <= ai <= 1e9)
//   Line 3: x (-1e18 <= x <= 1e18)

use std::io::{self, BufRead};

fn pi(a: &[i64], len: usize, i:usize, x: i64, flag: bool) -> bool {
    if i==len {
        return  flag;
    }
    let mut local_flag= flag;
    if a[i] == x {
        print!("{} ", i+1);
        local_flag = true;
    }
    pi(a, len, i+1, x, local_flag)
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

    // Line 3: x
    let x: i64 = lines
        .next()
        .expect("missing x")
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    if !pi(&a, n, 0, x, false) {
        print!("-1");
    }
}
