// B. Is It Sorted?
// https://codeforces.com/group/DNruwPkRNA/contest/726841/problem/B
//
// Input
//   Line 1: n q (1 <= n, q <= 100000)
//   Line 2: n integers a1 .. an (1 <= ai <= 1e9)
//   Next q lines: l r (1 <= l <= r <= n)

use std::io::{BufRead, stdin};

fn main() {
    let mut tokens = stdin()
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

    let mut p_arr: Vec<i64> = vec![0; n];
    let mut i = 1;
    while i < n {
        p_arr[i] = p_arr[i - 1] + if a[i - 1] <= a[i] { 1 } else { 0 }; //prefix added directly to contribution part
        i += 1;
    }

    queries.iter().for_each(|(l, r)| {
        let sum = p_arr[*r] - p_arr[*l];

        if sum == *r as i64 - *l as i64 {
            println!("YES");
        } else {
            println!("NO");
        }
    });
}
