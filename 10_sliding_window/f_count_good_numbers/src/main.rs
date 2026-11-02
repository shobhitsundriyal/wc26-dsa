// F. Count Good Numbers
// https://codeforces.com/group/DNruwPkRNA/contest/698784/problem/F
//
// Input
//   Line 1: n m k (1 <= k <= n <= 1e5, 1 <= m <= 1e5)
//   Line 2: n integers a1 .. an (1 <= ai <= 1e9)
//   Line 3: m integers, the good numbers (1 <= x <= 1e9)

use std::{collections::HashSet, io::{self, BufRead}};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n m k
    let first = lines.next().expect("missing first line").unwrap();
    let mut first = first.split_whitespace();

    let n: usize = first.next().unwrap().parse().unwrap();
    let m: usize = first.next().unwrap().parse().unwrap();
    let k: usize = first.next().unwrap().parse().unwrap();

    // Line 2 (and any following lines, in case it wraps): the n array values.
    let mut a: Vec<i64> = Vec::with_capacity(n);
    while a.len() < n {
        let line = lines.next().expect("not enough array values").unwrap();
        a.extend(
            line.split_whitespace()
                .map(|t| t.parse::<i64>().unwrap()),
        );
    }

    // Line 3 (and any following lines, in case it wraps): the m good numbers.
    let mut good: Vec<i64> = Vec::with_capacity(m);
    while good.len() < m {
        let line = lines.next().expect("not enough good numbers").unwrap();
        good.extend(
            line.split_whitespace()
                .map(|t| t.parse::<i64>().unwrap()),
        );
    }

    let good_set:HashSet<i64> = good.into_iter().collect();
    let contri: Vec<i64> = a.iter().map(|e| if good_set.contains(e) {1} else {0}).collect();

    let mut window_count: i64 = contri[..k].iter().sum();
    print!("{window_count} ");

    for i in k..n {
        window_count += contri[i] - contri[i-k];
        print!("{window_count} ");
    }
}
