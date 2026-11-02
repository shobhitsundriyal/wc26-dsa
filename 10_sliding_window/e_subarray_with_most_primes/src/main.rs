// E. Subarray with Most Primes
// https://codeforces.com/group/DNruwPkRNA/contest/698784/problem/E
//
// Input
//   Line 1: n k (1 <= k <= n <= 1e5)
//   Line 2: n integers a1 .. an (1 <= ai <= 1e5)

use std::io::{self, BufRead};

fn is_prime(x: f32) -> bool {
    if x==1.0 {
        return false;
    }
    let sqrt = x.sqrt();
    let mut i=2.0;
    let mut count=1;
    while i<=sqrt {
        if x%i==0.0 {
            count += 1;
            break;
        }
        i+=1.0;
    }
    count==1
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n k
    let first = lines.next().expect("missing first line").unwrap();
    let mut first = first.split_whitespace();

    let n: usize = first.next().unwrap().parse().unwrap();
    let k: usize = first.next().unwrap().parse().unwrap();

    // Line 2 (and any following lines, in case it wraps): the n array values.
    let mut a: Vec<u64> = Vec::with_capacity(n);
    while a.len() < n {
        let line = lines.next().expect("not enough array values").unwrap();
        a.extend(
            line.split_whitespace()
                .map(|t| t.parse::<u64>().unwrap()),
        );
    }

    let contri:Vec<i32> = a.iter().map(|e| if is_prime(*e as f32) {1} else {0}).collect();

    let mut window:i32 = contri[..k].iter().sum();
    let mut max = window;
    let mut idx = k;

    for i in k..n {
        window += contri[i] - contri[i-k];
        if max < window {
            max = window;
            idx = i+1;
        }
    }
    // println!("{} {}", idx-k, idx);
    a[idx-k..idx].iter().for_each(|e| print!("{e} "));
}
