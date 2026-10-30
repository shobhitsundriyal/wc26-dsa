// G. First Non-Repeating Character
// https://codeforces.com/group/DNruwPkRNA/contest/728252/problem/G
//
// Input
//   Line 1: s (1 <= |s| <= 1e5), lowercase English letters

use std::io::{self, BufRead};

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).unwrap();

    let s = input.trim();

    let mut freq = [0;26];

    for ch in s.chars(){
        freq[(ch as u8 - 97) as usize] += 1; 
    }

    let mut ans = String::from("-1");
    for ch in s.chars() {
        if freq[(ch as u8 - 97) as usize] == 1 {
            ans = ch.to_string();
            break;
        }
    }
    println!("{ans}");
}
