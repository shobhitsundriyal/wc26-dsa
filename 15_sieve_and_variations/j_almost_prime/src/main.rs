// J. Almost Prime
// https://codeforces.com/group/DNruwPkRNA/contest/733064/problem/J
//
// Input
//   Line 1: n (1 <= n <= 5 * 1e6)
//
// Idea:
//   A number is "almost prime" if it has exactly two distinct prime factors.
//   Build omega[x] = number of distinct prime factors of x using a sieve,
//   then count how many x in [1, n] have omega[x] == 2.
//
//   (The old version printed the number of distinct prime factors of n
//    itself, instead of counting almost primes up to n.)

use std::io::{self, BufRead};

fn main() {
    // Read exactly one line. We do NOT use read_to_string here, because it
    // waits for end-of-file and makes the program look stuck when typing
    // the number in a terminal.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut line = String::new();

    reader.read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();

    // omega[x] = number of distinct prime factors of x.
    // u8 is enough: for n <= 5e6 the maximum is 7.
    let mut omega = vec![0u8; n + 1];

    for p in 2..=n {
        // If omega[p] is still 0, no smaller prime divided p,
        // which means p itself is prime.
        if omega[p] == 0 {
            // p is a new distinct prime factor for all of its multiples.
            let mut multiple = p;
            while multiple <= n {
                omega[multiple] += 1;
                multiple += p;
            }
        }
    }

    // Count the numbers in [1, n] with exactly two distinct prime factors.
    let mut answer = 0;
    for x in 1..=n {
        if omega[x] == 2 {
            answer += 1;
        }
    }

    println!("{answer}");
}
