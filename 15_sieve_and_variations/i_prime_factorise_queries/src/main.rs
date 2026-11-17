// I. Prime Factorise Queries
// https://codeforces.com/group/DNruwPkRNA/contest/733064/problem/I
//
// Input
//   Line 1: q (1 <= q <= 1e6)
//   Next q lines: one integer x (1 <= x <= 1e6)

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: q
    let first = lines.next().expect("missing first line").unwrap();
    let q: usize = first.trim().parse().unwrap();

    // Next q lines: one query value each.
    let mut queries: Vec<usize> = Vec::with_capacity(q);
    while queries.len() < q {
        let line = lines.next().expect("not enough queries").unwrap();
        queries.extend(
            line.split_whitespace()
                .map(|t| t.parse::<usize>().unwrap()),
        );
    }
    let mx = *queries.iter().max().unwrap();
    let sq = (mx as f64).sqrt().floor() as usize +1;

    let mut factors = vec![0;sq+1];
    
        for i in 1..=sq {
            let mut j =i;
            while j<=sq {
                factors[j] += 1;
                j += i;
            }
        }

    queries.iter().for_each(|n| {

        let mut n: usize = *n;
    
        let sq = (n as f64).sqrt().floor() as usize +1;
    
        
    
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
    
        if n > 1 {print!("{}^{}", n, 1)}
        println!();
    });
}
