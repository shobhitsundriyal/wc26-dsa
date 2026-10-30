// F. Count Occurences
// https://codeforces.com/group/DNruwPkRNA/contest/728252/problem/F
//
// Input
//   Line 1: n (1 <= n <= 1e6)
//   Line 2: n integers a1 .. an (1 <= ai <= 1000)

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut a: Vec<usize> = Vec::with_capacity(n);
    while a.len() < n {
        let line = lines.next().unwrap().unwrap();
        for tok in line.split_whitespace() {
            a.push(tok.parse().unwrap());
        }
    }

    let mut freq = [0;1001];

    a.iter().for_each(|e| {
        freq[*e] += 1;
    });
    freq.iter().enumerate().for_each(|(idx, e)| {
        if *e>0 {
            println!("{} {}", idx, e)
        }
    });
}
