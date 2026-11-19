// Q. Last Occurence of X
// https://codeforces.com/group/DNruwPkRNA/contest/733421/problem/Q
//
// Input
//   Line 1: n (1 <= n <= 1e5)
//   Line 2: n integers a1 .. an (-1e9 <= ai <= 1e9)
//   Line 3: x (-1e18 <= x <= 1e18)

use std::io::{self, BufRead};

fn lo(a: &[i64], n:usize, x: i64) -> i64 {
    if n==0 {
        return  -1;
    }
    if a[n] == x {
        n as i64
    } else {
        lo(a, n-1, x)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: usize = first.trim().parse().unwrap();

    // Line 2 (and any following lines, in case it wraps): the n values.
    let mut a: Vec<i64> = Vec::with_capacity(n+1);
    a.push(0);
    while a.len() < n+1 {
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

        println!("{}", lo(&a, n, x));

}
