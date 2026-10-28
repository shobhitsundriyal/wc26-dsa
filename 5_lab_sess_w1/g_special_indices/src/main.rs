// G. Special Indices
// https://codeforces.com/group/DNruwPkRNA/contest/726841/problem/G
//
// Input
//   Line 1: n (1 <= n <= 1e5)
//   Line 2: n integers a0 .. a(n-1) (1 <= ai <= 1e4)

use std::io::{self, BufRead};

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

    let mut left_odd_sum = 0;
    let mut right_odd_sum = 0;
    let mut left_even_sum = 0;
    let mut right_even_sum = 0;
    a.iter().enumerate().for_each(|(idx, e)| {
        if idx.is_multiple_of(2) {
            right_even_sum += e;
        } else {
            right_odd_sum += e;
        }
    });

    let mut i = 0;
    let mut ans = 0;
    while i<n {
        if i.is_multiple_of(2) {
            right_even_sum -= a[i];
        } else {
            right_odd_sum -= a[i];
        }
        let even_sum = left_even_sum + right_odd_sum;
        let odd_sum = left_odd_sum + right_even_sum;

        if even_sum == odd_sum {
            ans += 1;
        }
        if i.is_multiple_of(2) {
            left_even_sum += a[i];
        } else {
            left_odd_sum += a[i];
        }
        i+=1;
    }
    println!("{ans}");
}
