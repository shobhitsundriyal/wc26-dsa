// O. Check If Array is Sorted
// https://codeforces.com/group/DNruwPkRNA/contest/733421/problem/O
//
// Input
//   Line 1: n (1 <= n <= 1e5)
//   Line 2: n integers a1 .. an (-1e9 <= ai <= 1e9)

use std::io::{self, BufRead};
fn sorted(a: &[i64], n: usize) -> bool {
    if n==0 {
        return  true;
    }
    if a[n] == a[n-1] {
        sorted(a, n-1)
    } else {
        false
    }
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

    if sorted(&a, n-1) {
        println!("YES");
    } else {
        println!("NO");
    }
}
