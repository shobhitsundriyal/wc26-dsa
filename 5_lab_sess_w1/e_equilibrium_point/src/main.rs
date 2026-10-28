// E. Equilibrium Point
// https://codeforces.com/group/DNruwPkRNA/contest/726841/problem/E
//
// Input
//   Line 1: n (3 <= n <= 1e5)
//   Line 2: n integers a0 .. a(n-1) (-1e4 <= ai <= 1e4)

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

    let mut i=0;
    // let mut p_arr = vec![a[0]];
    // //prefix sum
    // while i<n {
    //     p_arr.push(p_arr[i-1] + a[i]);
    //     i+=1;
    // }
    // i=0;
    // println!("{:?}", p_arr); 
    let mut ans: isize = -1;
    let mut left_side_sum = 0;
    let mut right_side_sum = a.iter().fold(0, |acc, e| acc+e);
    while i<n {
        if left_side_sum == right_side_sum - a[i] {
            ans = i as isize;
            break;
        }
        left_side_sum += a[i];
        right_side_sum -= a[i];
        i+=1;
    }
    println!("{ans}")
}
//O(1) space without prefix-sum
