// C. Count Good Numbers
// https://codeforces.com/group/DNruwPkRNA/contest/698784/problem/C
//
// Input
//   Line 1: n m k (1 <= k <= n <= 1e5, 1 <= m <= 1e5)
//   Line 2: n integers a1 .. an (1 <= ai <= 1e9)

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut tokens = stdin
        .lock()
        .lines()
        .flat_map(|line| line.unwrap().split_whitespace().map(str::to_string).collect::<Vec<_>>());

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let k: usize = tokens.next().unwrap().parse().unwrap();
    let t: usize = tokens.next().unwrap().parse().unwrap();

    let a: Vec<i64> = (0..n)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();

    let mut window:i64 = a[..k].iter().sum();
    let good_sum = (t*k) as i64;

    let mut count =0;
    if window >= good_sum {
        count +=1;
    }

    for i in k..n {
        window += a[i] - a[i-k];
        if window >= good_sum {
            count +=1;
        }
    }
    println!("{count}");
}
