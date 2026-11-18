// I. Nth Fibonacci
// https://codeforces.com/group/DNruwPkRNA/contest/733421/problem/I
//
// Input
//   Line 1: n (1 <= n <= 20)

use std::io::{self, BufRead};

fn fib(n: usize) -> usize {
    if n ==0 {return 0;}
    if n ==1 {return 1;}

    fib(n-1) + fib(n-2)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: usize = first.trim().parse().unwrap();

    println!("{}", fib(n-1));
}
