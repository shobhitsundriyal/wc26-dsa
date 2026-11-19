// S. Tower Of Hanoi
// https://codeforces.com/group/DNruwPkRNA/contest/733421/problem/S
//
// Input
//   Line 1: n (1 <= n <= 20)

use std::io::{self, BufRead};

fn toi(n:usize, a:char, b: char, c: char) {
    if n==0 {
        return;
    }
    toi(n-1, a, c, b);
    println!("Move {} to {}", a,c);
    toi(n-1, b, a, c);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: usize = first.trim().parse().unwrap();

    toi(n, 'A', 'B', 'C');
}
