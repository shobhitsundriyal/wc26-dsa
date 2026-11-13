// B. Ship Packages Within D Days
// https://codeforces.com/group/DNruwPkRNA/contest/730116/problem/B
//
// Input
//   Line 1: n d (1 <= n <= 2e5, 1 <= d <= n)
//   Line 2: n integers w1 .. wn (1 <= wi <= 1e9)
 
use std::io::{self, BufRead};

fn is_possible(arr: &[u64], capacity: u64, days_allowed: usize) -> bool {
    let mut days = 1;
    let mut cap = arr[0];

    let mut i =1;
    let n = arr.len();
    while i<n {
        if cap + arr[i] <= capacity {
            cap += arr[i];
        } else {
            cap = arr[i];
            days += 1;
        }
        i+=1;
    }

    days <= days_allowed
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n d
    let first = lines.next().expect("missing first line").unwrap();
    let mut first = first.split_whitespace();

    let n: usize = first.next().unwrap().parse().unwrap();
    let d: usize = first.next().unwrap().parse().unwrap();

    // Line 2 (and any following lines, in case it wraps): the n weights.
    let mut w: Vec<u64> = Vec::with_capacity(n);
    while w.len() < n {
        let line = lines.next().expect("not enough weights").unwrap();
        w.extend(
            line.split_whitespace()
                .map(|t| t.parse::<u64>().unwrap()),
        );
    }

    let mut l = *w.iter().max().unwrap();
    let mut r: u64 = w.iter().sum();

    let mut ans = r;
    while l<=r {
        let mid = (l+r)/2;
        if is_possible(&w,mid, d) {
            r = mid -1;
            ans = mid;
        } else {
            l = mid +1;
        }
    }

    println!("{ans}");
}
