// C. Count Vowels
// https://codeforces.com/group/DNruwPkRNA/contest/726133/problem/C

use std::io::{self, BufRead};

fn main() {
    // Read tokens lazily, line by line, so we do not wait for EOF.
    let stdin = io::stdin();
    let mut tokens = stdin.lock().lines().flat_map(|line| {
        line.unwrap()
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>()
    });

    // n, followed by the string s of length n
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let s: String = tokens.next().unwrap();

    // q, followed by q queries (l, r), 1-indexed and inclusive
    let q: usize = tokens.next().unwrap().parse().unwrap();
    let queries: Vec<(usize, usize)> = (0..q)
        .map(|_| {
            let l: usize = tokens.next().unwrap().parse().unwrap();
            let r: usize = tokens.next().unwrap().parse().unwrap();
            (l, r)
        })
        .collect();

    //todo
    let mut p = vec![0];
    for (i, c) in (0..).zip(s.chars()) {
        if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
            p.push(p[i] + 1);
        } else {
            p.push(p[i]);
        }
    }
    // println!("{:?}", p);
    queries.iter().for_each(|(l, r)| println!("{}", p[*r] - p[*l-1]));

}
