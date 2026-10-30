// B. Digit Frequency
// https://codeforces.com/group/DNruwPkRNA/contest/728252/problem/B
//
// Input
//   Line 1: N (0 <= N <= 10^100000), given without leading zeros
//           (except N = 0), so read it as a string.

use std::{collections::HashMap, io::{self, BufRead}};

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).unwrap();

    let s = input.trim();

    let mut map:HashMap<char, i64> = HashMap::new();
    for char in s.chars() {
        map.insert(char, map.get(&char).unwrap_or(&0) + 1);
    }

    let mut i =0;
    while i<10 {
        let ch = char::from_digit(i, 10).unwrap();
        print!("{} ", map.get(&ch).unwrap_or(&0));
        i+=1;
    }
}
