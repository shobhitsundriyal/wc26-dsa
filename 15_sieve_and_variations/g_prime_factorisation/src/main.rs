// G. Prime Factorisation
// https://codeforces.com/group/DNruwPkRNA/contest/733064/problem/G
//
// Input
//   Line 1: n (1 <= n <= 1e12)

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let mut n: usize = first.trim().parse().unwrap();

    let sq = (n as f64).sqrt().floor() as usize +1;
    let mut i=0;

    let mut factors = vec![0;sq+1];

    for i in 1..=sq {
        let mut j =i;
        while j<=sq {
            factors[j] += 1;
            j += i;
        }
    }

    for i in 1..=sq {
        if factors[i] == 2 && n.is_multiple_of(i) {
            let mut count = 0;
            while n.is_multiple_of(i) {
                n /= i;
                count += 1;
            }
            print!("{}^{} ", i, count);
        }
    }

    if n > 1 {println!("{}^{}", n, 1)}

}
