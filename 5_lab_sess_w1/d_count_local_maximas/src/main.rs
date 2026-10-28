// D. Count Local Maximas
// https://codeforces.com/group/DNruwPkRNA/contest/726841/problem/D
//
// Input
//   Line 1: n q (1 <= n, q <= 1e5)
//   Line 2: n integers a1 .. an (1 <= ai <= 1e9)
//   Next q lines: l r (1 <= l <= r <= n)

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    // Read lines lazily instead of slurping all of stdin, so the program
    // stops after the last query instead of waiting for EOF.
    let mut tokens = stdin
        .lock()
        .lines()
        .flat_map(|line| line.unwrap().split_whitespace().map(String::from).collect::<Vec<_>>());

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let q: usize = tokens.next().unwrap().parse().unwrap();

    let a: Vec<i64> = (0..n)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();

    let queries: Vec<(usize, usize)> = (0..q)
        .map(|_| {
                let l: usize = tokens.next().unwrap().parse().unwrap();
                let r:usize = tokens.next().unwrap().parse().unwrap();
                (l-1,r-1)
        })
        .collect();

    // p_arr[i] = number of local maxima among indices 1..i (0-based).
    // p_arr[0] = 0 is the sentinel. Indices 0 and n-1 can never be local
    // maxima because they lack a neighbour on one side.
    let mut p_arr: Vec<i64> = vec![0; n];
    let mut i = 1;
    while i < n - 1 {
        p_arr[i] = p_arr[i - 1] + if a[i - 1] < a[i] && a[i] > a[i + 1] { 1 } else { 0 };
        i += 1;
    }

    queries.iter().for_each(|(l, r)| {
        // Interior indices of [l, r] are l+1 .. r-1, so a range with
        // fewer than 3 elements contains no local maxima.
        let count = if r - l < 2 {
            0
        } else {
            p_arr[*r - 1] - p_arr[*l]
        };

        println!("{count}")
    });
}
