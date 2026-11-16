// E. Count Subarrays with At Most K Distinct
// https://codeforces.com/group/DNruwPkRNA/contest/732301/problem/E
//
// Input
//   Line 1: n k (1 <= n <= 1e5, 1 <= k <= n)
//   Line 2: n integers a1 .. an (1 <= ai <= 1e9)

use std::{collections::HashMap, io::{self, BufRead}};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n k
    let first = lines.next().expect("missing first line").unwrap();
    let mut first = first.split_whitespace();

    let n: usize = first.next().unwrap().parse().unwrap();
    let k: usize = first.next().unwrap().parse().unwrap();

    // Line 2 (and any following lines, in case it wraps): the n values.
    let mut a: Vec<u64> = Vec::with_capacity(n);
    while a.len() < n {
        let line = lines.next().expect("not enough array values").unwrap();
        a.extend(
            line.split_whitespace()
                .map(|t| t.parse::<u64>().unwrap()),
        );
    }

    let mut map:HashMap<u64, u64> = HashMap::new();
    let mut l=0;
    let mut count = 0;
    for r in 0..n{
        map.entry(a[r]).and_modify(|count| *count += 1).or_insert(1);

        while map.len() > k {
            map.entry(a[l]).and_modify(|count| *count -=1);
            if map[&a[l]] == 0 {
                map.remove(&a[l]);
            }
            l += 1;
        }

        count += r-l+1;
    }

    println!("{count}");
}
