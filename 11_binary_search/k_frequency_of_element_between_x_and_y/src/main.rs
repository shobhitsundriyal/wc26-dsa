// K. Frequency of Element b/w X and Y
// https://codeforces.com/group/DNruwPkRNA/contest/729595/problem/K
//
// Input
//   Line 1: n q (1 <= n, q <= 1e5)
//   Line 2: n integers a1 .. an (-1e9 <= ai <= 1e9)
//   Next q lines: two integers x and y (-1e9 <= x, y <= 1e9)

use std::io::{self, BufRead};

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
    // Next q lines: two values each (x y).
    let mut values: Vec<i64> = Vec::with_capacity(2 * q);
    while values.len() < 2 * q {
        let line = lines.next().expect("not enough queries").unwrap();
        values.extend(
            line.split_whitespace()
                .map(|t| t.parse::<i64>().unwrap()),
        );
    }

    let queries: Vec<(i64, i64)> = values
        .chunks_exact(2)
        .map(|pair| (pair[0], pair[1]))
        .collect();

    queries.iter().for_each(|(x,y)| {
        let mut l=0;
        let mut r = n-1;
        let mut idx1: isize  = -1;

        while l<=r {
            let mid = l + ((r-l) /2);

            if *x <= a[mid] {
                idx1 = mid as isize;
                let ir = mid as isize - 1;
                if ir < 0{
                    break;
                }
                r = mid -1;
            } else {
                l = mid + 1;
            }

        }
        // 2nd itr
        let mut l=0;
        let mut r = n-1;
        let mut idx2: isize  = -1 as isize;

        while l<=r {
            let mid = l + ((r-l) /2);

            if a[mid] <= *y {
                idx2 = mid as isize;
                l = mid + 1;
            } else {
                let ir = mid as isize - 1;
                if ir < 0{
                    break;
                }
                r = mid -1;
            }

        }
        
        if idx1 == -1 || idx2 == -1 || idx2 < idx1 {
            println!("0");
        } else {
            println!("{}", idx2 - idx1 + 1);
        }
        // println!("{} . {}",idx1, idx2);
    });
}
