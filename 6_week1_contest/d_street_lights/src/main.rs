// D. Street Lights
// https://codeforces.com/group/DNruwPkRNA/contest/727638/problem/D
//
// Input (matches the provided template)
//   Line 1: n (1 <= n <= 1e5)
//   Line 2: n integers power[0] .. power[n-1] (0 <= power[i] <= n)
//
// Output
//   A single integer: the number of spots no lamp lights.

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    // Read tokens lazily instead of slurping all of stdin, so the program
    // stops after the n values instead of waiting for EOF.
    let mut tokens = stdin
        .lock()
        .lines()
        .flat_map(|line| line.unwrap().split_whitespace().map(String::from).collect::<Vec<_>>());

    let n: usize = tokens.next().unwrap().parse().unwrap();

    let power: Vec<usize> = (0..n)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();

    // TODO: solve
    let mut left_arr = vec![0;n];
    let mut right_arr = vec![0;n];

    let mut i = 0;
    let mut remaining=0;
    while i<n {
        if remaining < power[i] {
            remaining = power[i];
            left_arr[i] = 1;
        } else if remaining > 0 {
            left_arr[i] = 1;
            if remaining != power[i] {
                remaining -=1;
            }
        }
        i+=1;
        // println!("{:?}", left_arr);
    }
    remaining = 0;
    i=0;
    while i<n {
        let idx = n-i-1;
        if remaining < power[idx] {
            remaining = power[idx];
            right_arr[idx] = 1;
        }else if remaining > 0 {
            right_arr[idx] = 1;
            if remaining != power[idx] {
                remaining -=1;
            }
        }
        i+=1;
    }
    i=0;
    let mut count =0;
    // println!("{:?}. {:?}", left_arr, right_arr);
    while i<n {
        count+= left_arr[i]|right_arr[i];
        i+=1;
    }
    println!("{}", n-count);
}
