// C. Longest Substring with At Most K Distinct Characters
// https://codeforces.com/group/DNruwPkRNA/contest/732301/problem/C
//
// Input
//   Line 1: n k (1 <= n <= 1e5, 1 <= k <= 52)
//   Line 2: s (uppercase and lowercase English letters, length n)

use std::{collections::HashMap, io::{self, BufRead}};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n k
    let first = lines.next().expect("missing first line").unwrap();
    let mut first = first.split_whitespace();

    let n: usize = first.next().unwrap().parse().unwrap();
    let k: usize = first.next().unwrap().parse().unwrap();

    // Line 2: s
    let mut s: Vec<char> = Vec::with_capacity(n);
    while s.len() < n {
        let line = lines.next().expect("not enough array values").unwrap();
        s.extend(
            line.chars()
                // .map(|t| t.parse::<char>().unwrap()),
        );
    }

    let mut map:HashMap<char, u64> = HashMap::new();
    let mut l=0;
    let mut best_len = 0;
    for r in 0..n{
        map.entry(s[r]).and_modify(|count| *count += 1).or_insert(1);

        while map.len() > k {
            map.entry(s[l]).and_modify(|count| *count -=1);
            if map[&s[l]] == 0 {
                map.remove(&s[l]);
            }
            l += 1;
        }

        best_len = best_len.max(r-l+1);
    }

    println!("{best_len}");
}
