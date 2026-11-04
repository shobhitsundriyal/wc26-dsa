// C. Binary Search (Strings)
// https://codeforces.com/group/DNruwPkRNA/contest/729595/problem/C
//
// Input
//   Line 1: n (1 <= n <= 1e5)
//   Next n lines: one name Si (lowercase English letters, length 1..20),
//                 given in non-decreasing lexicographical order
//   Last line: a name x to search for

use std::{cmp::Ordering, io::{self, BufRead}};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: usize = first.trim().parse().unwrap();

    // Next n lines: the sorted names.
    let mut names: Vec<String> = Vec::with_capacity(n);
    while names.len() < n {
        let line = lines.next().expect("not enough names").unwrap();
        names.push(line.trim().to_string());
    }

    // Last line: the name to search for.
    let query = lines
        .next()
        .expect("missing query name")
        .unwrap()
        .trim()
        .to_string();

    let mut l: usize=0;
    let mut r= n-1;
    let mut ans = "NO".to_string();

    while l<=r {
        let mid = (l+r) /2;
        match names[mid].cmp(&query) {
            Ordering::Equal => {
                ans = "YES".to_string();
                break;
            },
            Ordering::Less => l = mid+1,
            Ordering::Greater => {
                let ir = mid as isize - 1;
                if ir < 0{
                    break;
                }
                r = mid-1;
            }
        }

    }
    println!("{ans}");
}
