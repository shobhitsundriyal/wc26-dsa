// F. Prime Range Queries
// https://codeforces.com/group/DNruwPkRNA/contest/733064/problem/F
//
// Input
//   Line 1: n q (1 <= n, q <= 1e6)
//   Next q lines: two integers l and r (1 <= l <= r <= n)

use std::{io::{self, BufRead}, vec};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n q
    let first = lines.next().expect("missing first line").unwrap();
    let mut first = first.split_whitespace();

    let n: usize = first.next().unwrap().parse().unwrap();
    let q: usize = first.next().unwrap().parse().unwrap();

    // Next q lines: two values each (l r).
    let mut values: Vec<usize> = Vec::with_capacity(2 * q);
    while values.len() < 2 * q {
        let line = lines.next().expect("not enough queries").unwrap();
        values.extend(
            line.split_whitespace()
                .map(|t| t.parse::<usize>().unwrap()),
        );
    }

    let queries: Vec<(usize, usize)> = values
        .chunks_exact(2)
        .map(|pair| (pair[0], pair[1]))
        .collect();

    let mut factors = vec![0;n+1];

    for i in 1..=n {
        let mut j =i;
        while j<=n {
            factors[j] += 1;
            j += i;
        }
    }

    let mut p_sum = vec![0;n+1];
    let mut i = 1;
    while i<=n {
        if factors[i] == 2 {
            p_sum[i] += p_sum[i-1] + 1; 
        } else {
            p_sum[i] += p_sum[i-1]; 
        }
        i+=1;
    } 

    queries.iter().for_each(|(l, r)| {
        // let mut count = 0;
        // for i in *l..=*r {
        //     if factors[i] == 2 {
        //         count += 1;``
        //     }
        // }
        println!("{}", p_sum[*r] - p_sum[*l-1]);
    });
    // println!("{:?}", p_sum);
}
