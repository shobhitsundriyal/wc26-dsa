// A. Search Queries
// https://codeforces.com/group/DNruwPkRNA/contest/729595/problem/A
//
// Input
//   Line 1: n q (1 <= n, q <= 1e5)
//   Line 2: n distinct integers a1 .. an, sorted strictly increasing
//   Next q lines: one integer x (-1e9 <= x <= 1e9)

use std::io::{self, BufRead};

fn bi_search(arr: &Vec<i64>, x:i64, n:usize) -> i64 {
    let mut l: usize=0;
    let mut r= n-1;

    while l<=r {
        let mid = (l+r) /2;
        if arr[mid] == x {
            return mid as i64;
        } else if arr[mid] < x  {
            l = mid+1;
        } else {
            let ir = mid as isize - 1;
            if ir < 0{
                break;
            }
            r = mid-1;
        }

    }
    return -2;
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n q
    let first = lines.next().expect("missing first line").unwrap();
    let mut first = first.split_whitespace();

    let n: usize = first.next().unwrap().parse().unwrap();
    let q: usize = first.next().unwrap().parse().unwrap();

    // Line 2 (and any following lines, in case it wraps): the n array values.
    let mut a: Vec<i64> = Vec::with_capacity(n);
    while a.len() < n {
        let line = lines.next().expect("not enough array values").unwrap();
        a.extend(
            line.split_whitespace()
                .map(|t| t.parse::<i64>().unwrap()),
        );
    }

    // Next q lines: one query value each.
    let mut queries: Vec<i64> = Vec::with_capacity(q);
    while queries.len() < q {
        let line = lines.next().expect("not enough queries").unwrap();
        queries.extend(
            line.split_whitespace()
                .map(|t| t.parse::<i64>().unwrap()),
        );
    }

    queries.iter().for_each(|x| println!("{}", bi_search(&a, *x, n) + 1));
}
