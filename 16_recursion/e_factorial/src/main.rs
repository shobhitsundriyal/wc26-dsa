// E. Factorial
// https://codeforces.com/group/DNruwPkRNA/contest/733421/problem/E
//
// Input
//   Line 1: n (0 <= n <= 20)

use std::io::{self, BufRead};

fn fact(n: usize) -> usize {
    if n <= 1 {return 1;}
    
    n * fact(n-1)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: usize = first.trim().parse().unwrap();

    println!("{}", fact(n));
}
