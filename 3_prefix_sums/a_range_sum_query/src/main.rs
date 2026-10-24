// A. Range Sum Query
// https://codeforces.com/group/DNruwPkRNA/contest/726133/problem/A

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

    // n, followed by the n array elements
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let a: Vec<i64> = (0..n)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();

    // q, followed by q queries (l, r), 1-indexed and inclusive
    let q: usize = tokens.next().unwrap().parse().unwrap();
    let queries: Vec<(usize, usize)> = (0..q)
        .map(|_| {
            let l: usize = tokens.next().unwrap().parse().unwrap();
            let r: usize = tokens.next().unwrap().parse().unwrap();
            (l, r)
        })
        .collect();

    // create prefix sum array
    let mut p:Vec<i64> = vec![];
    p.push(a[0]);
    let mut i=1;
    while i<n {
        p.push(a[i]+p[i-1]);
        i+=1;
    }

    queries.iter().for_each(|(l, r)| if *l!=1 {println!("{}", p[*r-1] - p[*l-2])} else {println!("{}", p[*r-1])} );
    

}
