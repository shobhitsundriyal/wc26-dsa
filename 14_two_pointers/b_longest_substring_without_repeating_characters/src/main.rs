// B. Longest Substring Without Repeating Characters
// https://codeforces.com/group/DNruwPkRNA/contest/732301/problem/B
//
// Input
//   Line 1: n (1 <= n <= 1e5)
//   Line 2: s (uppercase and lowercase English letters, length n)

use std::{collections::HashSet, io::{self, BufRead}};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: usize = first.trim().parse().unwrap();

    // Line 2: s
    let s = lines
        .next()
        .expect("missing string")
        .unwrap()
        .trim()
        .to_string();

    let mut l = 0;
    let mut best_len = 0;
    let mut set:HashSet<char> = HashSet::new();
    for (r, ch) in s.chars().enumerate() {
        // println!("{:?}", set);
        while set.contains(&ch) {
            set.remove(&s.chars().nth(l).unwrap());
            l += 1;
        } 
        set.insert(ch);
        best_len = best_len.max(r-l+1);
    }
    println!("{best_len}");
}
