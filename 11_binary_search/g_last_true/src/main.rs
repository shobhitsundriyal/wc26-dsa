// G. Last True
// https://codeforces.com/group/DNruwPkRNA/contest/729595/problem/G
//
// Interactive problem.
//
// Input
//   Line 1: n (1 <= n <= 1e18)
//
// After reading n, the hidden sequence is queried by printing "? i" and
// reading one line ("T" or "F"). The final answer is printed as "! x",
// where x is the last position containing T (or 0 if none exists).

use std::{cmp::Ordering, io::{self, BufRead, Write}};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: u64 = first.trim().parse().unwrap();

    let mut l=1;
    let mut r = n;
    let mut ans = 0;

    while l<=r {
        let mid = l + ((r-l)/2);

        println!("? {}", mid);
        io::stdout().flush().unwrap();
        let resp = lines.next().unwrap().unwrap();

        if resp.cmp(&"T".to_string()) == Ordering::Equal {
            ans = mid;
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }
    println!("! {}", ans);
}
