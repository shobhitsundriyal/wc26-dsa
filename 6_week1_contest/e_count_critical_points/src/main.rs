// E. Count Critical Points
// https://codeforces.com/group/DNruwPkRNA/contest/727638/problem/E
//
// Input (matches the provided template)
//   Line 1: n q (1 <= n, q <= 1e5)
//   Line 2: n integers a[0] .. a[n-1] (1 <= a[i] <= 1e9)
//   Next q lines: L R (1 <= L <= R <= n), 1-based and inclusive
//
// Output
//   One line per query: the number of critical points at positions i with
//   L < i < R (the endpoints are never counted).

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    // Read tokens lazily instead of slurping all of stdin, so the program
    // stops after the q queries instead of waiting for EOF.
    let mut tokens = stdin
        .lock()
        .lines()
        .flat_map(|line| line.unwrap().split_whitespace().map(String::from).collect::<Vec<_>>());

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let q: usize = tokens.next().unwrap().parse().unwrap();

    let a: Vec<i64> = (0..n)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();

    // Queries stay 1-based, as numbered in the statement.
    let queries: Vec<(usize, usize)> = (0..q)
        .map(|_| {
            let l: usize = tokens.next().unwrap().parse().unwrap();
            let r: usize = tokens.next().unwrap().parse().unwrap();
            (l-1, r-1)
        })
        .collect();

    // TODO: solve
    let mut p_arr: Vec<u32> = vec![];
    let mut i =0;

    while i<n {
        if i==0 || i==n-1 {
            p_arr.push(0);
        } else {
            if (a[i-1] < a[i] && a[i+1] < a[i]) || (a[i-1] > a[i] && a[i+1] > a[i]) {
                p_arr.push(1);
            } else {
                p_arr.push(0);
            }
        }
        i+=1;
    }
    i=1;
    while i<n {
        p_arr[i] += p_arr[i-1];
        i+=1;
    }
    // println!("{:?}", p_arr);
    queries.iter().for_each(|(l,r)| {
        println!("{}", if *r <= *l+1 {0} else {p_arr[*r-1] - p_arr[*l]});
    });
}
