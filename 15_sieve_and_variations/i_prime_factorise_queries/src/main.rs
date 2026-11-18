// I. Prime Factorise Queries
// https://codeforces.com/group/DNruwPkRNA/contest/733064/problem/I
//
// Input
//   Line 1: q (1 <= q <= 1e6)
//   Next q lines: one integer x (1 <= x <= 1e6)
//
// Idea:
//   Precompute the smallest prime factor (spf) for every value up to the
//   largest query. Then each query is factorised by repeatedly dividing
//   out spf[x], which takes only O(number of prime factors) per query.
//
//   (The old version scanned up to sqrt(x) for every query, which is about
//    1e9 operations in total and is too slow.)

use std::fmt::Write as _;
use std::io::{self, BufRead};

fn main() {
    // Read line by line, and stop as soon as q values have been read.
    // We do NOT use read_to_string here: it waits for end-of-file, so when
    // typing input in a terminal the program looks like it hangs forever.
    // The String buffer is reused, so there is no allocation per line.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut line = String::new();

    // Line 1: q
    reader.read_line(&mut line).unwrap();
    let q: usize = line.trim().parse().unwrap();

    // Next q lines: one query value each.
    let mut queries: Vec<usize> = Vec::with_capacity(q);
    while queries.len() < q {
        line.clear();
        if reader.read_line(&mut line).unwrap() == 0 {
            break; // reached end of input
        }
        for token in line.split_whitespace() {
            queries.push(token.parse().unwrap());
        }
    }

    // We only need to sieve up to the largest query value.
    let max_value = *queries.iter().max().unwrap();

    // spf[v] = smallest prime factor of v.
    // spf[v] is 0 while v has not been assigned a factor yet.
    let mut spf = vec![0usize; max_value + 1];

    let mut p = 2;
    while p <= max_value {
        if spf[p] == 0 {
            // No smaller prime divided p, so p itself is prime.
            spf[p] = p;

            // Mark p as the smallest prime factor of its multiples.
            // Start at p * p, because any smaller multiple already has a
            // smaller prime factor. Only set spf if it is still 0.
            let mut multiple = p * p;
            while multiple <= max_value {
                if spf[multiple] == 0 {
                    spf[multiple] = p;
                }
                multiple += p;
            }
        }
        p += 1;
    }

    // Build one line per query and print it with println!.
    // The String is reused, so we do not allocate a new one every time.
    let mut output_line = String::new();

    for &value in &queries {
        output_line.clear();
        let mut remaining = value;
        let mut first = true;

        // Factorise "remaining" one prime at a time.
        while remaining > 1 {
            let prime = spf[remaining];

            // Count how many times this prime divides the value.
            let mut exponent = 0;
            while remaining % prime == 0 {
                remaining /= prime;
                exponent += 1;
            }

            if !first {
                output_line.push(' ');
            }
            write!(output_line, "{}^{}", prime, exponent).unwrap();
            first = false;
        }

        // For value == 1 the line is empty, which is the correct output.
        println!("{output_line}");
    }
}
