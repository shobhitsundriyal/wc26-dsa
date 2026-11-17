// E. Good Number Queries
// https://codeforces.com/group/DNruwPkRNA/contest/733064/problem/E
//
// Input
//   Line 1: q (1 <= q <= 1e6)
//   Next q lines: one integer x (1 <= x <= 1e6)

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: q
    let first = lines.next().expect("missing first line").unwrap();
    let q: usize = first.trim().parse().unwrap();

    // Next q lines: one query value each.
    let mut queries: Vec<usize> = Vec::with_capacity(q);
    while queries.len() < q {
        let line = lines.next().expect("not enough queries").unwrap();
        queries.extend(
            line.split_whitespace()
                .map(|t| t.parse::<usize>().unwrap()),
        );
    }

    let n = *queries.iter().max().unwrap();
    let mut factors = vec![0;n+1];

    for i in 1..=n {
        let mut j =i;
        while j<=n {
            factors[j] += 1;
            j += i;
        }
    }

    queries.iter().for_each(|e| {
        let count = factors[*e];
        if factors[count] == 2 {
            println!("YES");
        } else {
            println!("NO");
        }
    });
}
