// F. First True
// https://codeforces.com/group/DNruwPkRNA/contest/729595/problem/F
//
// Interactive problem.
//
// Input
//   Line 1: n (1 <= n <= 1e18)
//
// After reading n, the hidden sequence is queried by printing "? i" and
// reading one line ("F" or "T"). The final answer is printed as "! x",
// where x is the first position containing T (or n + 1 if none exists).

use std::{cmp::Ordering, io::{self, BufRead, Write}};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: u64 = first.trim().parse().unwrap();


    // TODO: solve
    //
    // Interact through `lines` and stdout:
    //   print "? i" (flush), then read the response line;
    //   finally print "! x" (flush).

    let mut l=1;
    let mut r = n;
    let mut ans = n+1;

    while l<=r {
        let mid = l + ((r-l)/2);

        println!("? {}", mid);
        io::stdout().flush().unwrap();
        let resp = lines.next().unwrap().unwrap();

        if resp.cmp(&"T".to_string()) == Ordering::Equal {
            ans = mid;
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }
    println!("! {}", ans);
}
