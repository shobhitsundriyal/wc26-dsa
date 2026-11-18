// D. Print Number in Reverse
// https://codeforces.com/group/DNruwPkRNA/contest/733421/problem/D
//
// Input
//   Line 1: n (0 <= n <= 1e18)

use std::io::{self, BufRead};
fn call(n: u64) {
    if n == 0 {return}

    print!("{}", n%10);
    call(n/10);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: u64 = first.trim().parse().unwrap();
    if n==0 {print!("0"); return }
    call(n);
}
