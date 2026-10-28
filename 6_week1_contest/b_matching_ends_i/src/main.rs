// B. Matching Ends - I
// https://codeforces.com/group/DNruwPkRNA/contest/727638/problem/B
//
// Input (matches the provided template)
//   Line 1: n (1 <= n <= 1000)
//   Line 2: n integers colours[0] .. colours[n-1] (1 <= colours[i] <= 1e6)
//
// Note: only the function countPrettyPieces is required in the official
// template. This Rust version reads the same stdin format and prints the
// single answer.

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    // Read tokens lazily instead of slurping all of stdin, so the program
    // stops after the n colours instead of waiting for EOF.
    let mut tokens = stdin
        .lock()
        .lines()
        .flat_map(|line| line.unwrap().split_whitespace().map(String::from).collect::<Vec<_>>());

    let n: usize = tokens.next().unwrap().parse().unwrap();

    let colours: Vec<u32> = (0..n)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();

    let mut i =0;
    // let mut j=0;
    let mut count =0;

    while i<n{
        let mut j=i;
        while j<n {
            if colours[i] == colours[j] {
                count += 1;
            }
            j+=1;
        }
        i+=1;
    }
    println!("{count}");
}
