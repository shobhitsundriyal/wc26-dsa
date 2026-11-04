// E. Frequency of Element
// https://codeforces.com/group/DNruwPkRNA/contest/729595/problem/E
//
// Input
//   Line 1: n q (1 <= n, q <= 1e5)
//   Line 2: n integers a1 .. an (-1e9 <= ai <= 1e9)
//   Next q lines: one integer x (-1e9 <= x <= 1e9)

use std::io::{self, BufRead};

fn bi_search_first(arr: &Vec<i64>, x:i64, n:usize) -> i64 {
    let mut l: usize=0;
    let mut r= n-1;
    let mut ans = -2;
    while l<=r {
        let mid = (l+r) /2;
        if arr[mid] == x {
            ans = mid as i64;
            let ir = mid as isize - 1;
            if ir < 0{
                break;
            }
            r = mid-1;
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
    return ans;
}

fn bi_search_last(arr: &Vec<i64>, x:i64, n:usize) -> i64 {
    let mut l: usize=0;
    let mut r= n-1;
    let mut ans = -2;

    while l<=r {
        let mid = (l+r) /2;
        if arr[mid] == x {
            ans = mid as i64;
            l = mid+1;
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
    return ans;
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
    a.sort();
    // Next q lines: one query value each.
    let mut queries: Vec<i64> = Vec::with_capacity(q);
    while queries.len() < q {
        let line = lines.next().expect("not enough queries").unwrap();
        queries.extend(
            line.split_whitespace()
                .map(|t| t.parse::<i64>().unwrap()),
        );
    }
    // println!("___________");
    queries.iter().for_each(|x| {
        let first = bi_search_first(&a, *x, n) +1;
        let last = bi_search_last(&a, *x, n) +1;
        if last == -1 {
            println!("0");
        } else {
            println!("{}", last-first+1);
        }
    });
}
