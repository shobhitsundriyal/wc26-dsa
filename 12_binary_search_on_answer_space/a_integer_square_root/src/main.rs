// A. Integer Square Root
// https://codeforces.com/group/DNruwPkRNA/contest/730116/problem/A
//
// Input
//   Line 1: n (0 <= n <= 1e18)

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: u128 = first.trim().parse().unwrap();

    let mut l: u128 =0;
    let mut r: u128 = n;
    let mut ans: u128 = 0;
    while l<=r {
        let mid = (l+r)/2;
        if mid * mid <= n {
            ans = mid;
            l = mid +1;
        } else {
            r = mid-1;
        }
    }

    println!("{ans}");
}
