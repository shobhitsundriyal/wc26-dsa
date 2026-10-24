// B. Even Sum Query
// https://codeforces.com/group/DNruwPkRNA/contest/726133/problem/B

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
    let mut a: Vec<i64> = (0..n)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect(); // so that we can do 1 indexed, here it would make it easier
    a.insert(0,0);

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
    while i<n+1 {
        if i.is_multiple_of(2) {
            p.push(a[i]+p[i-1]);
        } else {
            p.push(p[i-1]);
        }
        i+=1;
    }
    // println!("{:?}", a);
    // println!("{:?}", p);
    // queries.iter().for_each(|(l, r)| if *l!=1 {println!("{}", p[*r] - p[*l])} else {println!("{}", p[*r-1])} );
    queries.iter().for_each(|(l, r)| println!("{}", p[*r] - p[*l-1]) );
    

}
