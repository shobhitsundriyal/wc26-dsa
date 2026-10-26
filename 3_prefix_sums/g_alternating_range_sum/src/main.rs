// G. Alternating Range Sum
// https://codeforces.com/group/DNruwPkRNA/contest/726133/problem/G

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

    // first line: n and q
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let q: usize = tokens.next().unwrap().parse().unwrap();
    // let x: usize = tokens.next().unwrap().parse().unwrap();

    // second line: the n array elements
    let mut a: Vec<i64> = (0..n)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();
    a.insert(0,0); // so that we can do 1 indexed, here it would make it easier

    // next q lines: q queries (l, r), 1-indexed and inclusive
    let queries: Vec<(usize, usize)> = (0..q)
        .map(|_| {
            let l: usize = tokens.next().unwrap().parse().unwrap();
            let r: usize = tokens.next().unwrap().parse().unwrap();
            (l, r)
        })
        .collect();

    // create prefix sum array
    let mut p1:Vec<i64> = vec![];
    p1.push(a[0]);
    let mut i =1;
    while i<n+1 {
        if i%2!=0 {
            p1.push(p1[i-1] - a[i]);   
        } else {   
            p1.push(p1[i-1] + a[i]);   
        }
        i+=1;
    }
    // println!("{:?}", a);
    // println!("{:?}", p1);
    queries.iter().for_each(|(l, r)| println!("{}", if *l%2==0 {
        p1[*r] - p1[*l-1]
    } else {
        -(p1[*r] - p1[*l-1])
    }) );
}
