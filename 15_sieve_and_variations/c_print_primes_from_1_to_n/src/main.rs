// C. Print Primes from 1 to N
// https://codeforces.com/group/DNruwPkRNA/contest/733064/problem/C
//
// Input
//   Line 1: n (1 <= n <= 1e6)

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: usize = first.trim().parse().unwrap();

    let mut factors = vec![0;n+1];

    for i in 1..=n {
        let mut j =i;
        while j<=n {
            factors[j] += 1;
            j += i;
        }
    }

    let mut count = 0;
    factors.iter().enumerate().for_each(|(i, f)| {
        if *f == 2 {
            print!("{i} ");
            count += 1;
        }
    });
    println!();
    println!("{count}");
}
