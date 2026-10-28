// H. Product of Array Except Self
// https://codeforces.com/group/DNruwPkRNA/contest/726841/problem/H
//
// Input
//   Line 1: n (2 <= n <= 1e5)
//   Line 2: n integers a1 .. an (-30 <= ai <= 30)

use std::{io::{self, BufRead}, process::exit};

fn main() {
    let stdin = io::stdin();

    // Read lines lazily instead of slurping all of stdin, so the program
    // stops after the last value instead of waiting for EOF.
    let mut tokens = stdin
        .lock()
        .lines()
        .flat_map(|line| line.unwrap().split_whitespace().map(String::from).collect::<Vec<_>>());

    let n: usize = tokens.next().unwrap().parse().unwrap();

    let a: Vec<i64> = (0..n)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();

    // TODO: solve
    let mut total_prod = 1;
    let mut zero_count = 0;
    a.iter().for_each(|e| {
        if *e!=0 {
            total_prod *= e;
        } else {
            zero_count+=1;
        }
    });

    if zero_count > 1 {
        a.iter().for_each(|_| print!("0 "));
        exit(0);
    }
    a.iter().for_each(|e| {
        if *e !=0 {
            print!("{} ", if zero_count==0 {total_prod/e} else {
                0
            })
        } else {
            print!("{} ", total_prod)
        }
    });
}
