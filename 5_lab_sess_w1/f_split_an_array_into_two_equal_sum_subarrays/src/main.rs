// F. Split an Array into Two Equal Sum Subarrays
// https://codeforces.com/group/DNruwPkRNA/contest/726841/problem/F
//
// Input
//   Line 1: n (1 <= n <= 1e5)
//   Line 2: n integers a1 .. an (1 <= ai <= 1e6)

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

    // with prefix sum O(N) space
    // let mut i=1;
    // let mut p_arr = vec![a[0]];
    // //prefix sum
    // while i<n {
    //     p_arr.push(p_arr[i-1] + a[i]);
    //     i+=1;
    // }
    // i=0;
    // // println!("{:?}", p_arr);
    // while i<n-1 {
    //     let left_side_sum = p_arr[i];
    //     let right_side_sum = p_arr[n-1] - p_arr[i];
    //     if left_side_sum == right_side_sum {
    //         println!("YES");
    //         exit(0);
    //     }
    //     i+=1;
    // }
    // println!("NO")

    // now O(1) space
    let mut i=1; 
    let mut ans: isize = -1;
    let mut left_side_sum = a[0];
    let mut right_side_sum = a.iter().fold(0, |acc, e| acc+e) - a[0];
    while i<n {
        if left_side_sum == right_side_sum {
            println!("YES");
            exit(0);
        }
        left_side_sum += a[i];
        right_side_sum -= a[i];
        i+=1;
    }
    println!("NO")
}
