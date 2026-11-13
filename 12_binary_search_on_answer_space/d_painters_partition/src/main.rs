// D. Painter's Partition
// https://codeforces.com/group/DNruwPkRNA/contest/730116/problem/D
//
// Input
//   Line 1: n k (1 <= n <= 2e5, 1 <= k <= n)
//   Line 2: n integers a1 .. an (1 <= ai <= 1e9)

use std::io::{self, BufRead};

fn is_possible(arr: &[u64], check_hr: u64, n_painters: u64) -> bool {
    let mut painter_req = 1;
    let mut t_time = 0;
    for time in arr {
        if t_time + *time <= check_hr {
            t_time += *time
        } else {
            t_time = *time;
            painter_req += 1;
        }
    }

    painter_req <= n_painters
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n k
    let first = lines.next().expect("missing first line").unwrap();
    let mut first = first.split_whitespace();

    let n: usize = first.next().unwrap().parse().unwrap();
    let k: u64 = first.next().unwrap().parse().unwrap();

    // Line 2 (and any following lines, in case it wraps): the n boards.
    let mut a: Vec<u64> = Vec::with_capacity(n);
    while a.len() < n {
        let line = lines.next().expect("not enough boards").unwrap();
        a.extend(
            line.split_whitespace()
                .map(|t| t.parse::<u64>().unwrap()),
        );
    }

    let mut l = *a.iter().max().unwrap();
    let mut r = a.iter().sum();
    let mut ans = r;

    while l<=r {
        let mid = (l+r)/2;
        if is_possible(&a,mid, k) {
            r = mid -1;
            ans = mid;
        } else {
            l = mid +1;
        }
    }

    println!("{ans}");
}
