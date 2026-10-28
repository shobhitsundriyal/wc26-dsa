// F. Matching Ends - II
// https://codeforces.com/group/DNruwPkRNA/contest/727638/problem/F
//
// Input (matches the provided template)
//   Line 1: n (1 <= n <= 1e5)
//   Line 2: n integers colours[0] .. colours[n-1] (1 <= colours[i] <= 1e6)
//
// Note: only the function countPrettyPieces is required in the official
// template. This Rust version reads the same stdin format and prints the
// single answer.

use std::{collections::HashMap, io::{self, BufRead}};

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
    let mut count: usize =n;
    let mut map:HashMap<u32, u32> = HashMap::new();
    while i<n{
        // let mut j=i;
        let key = &colours[i];
        if map.contains_key(key) {
            let update_value = *map.get(key).unwrap() + 1;
            map.insert(*key, update_value);
        } else {
            map.insert(*key, 0);
        }
        i+=1;
    }

    for (_, v) in map {
        let v = v as usize;
        count += (v*(v + 1)) / 2;
    }
    println!("{count}");
}
