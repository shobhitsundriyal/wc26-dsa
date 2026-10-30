// A. Character Frequency
// https://codeforces.com/group/DNruwPkRNA/contest/728252/problem/A
//
// Input
//   Line 1: s (1 <= |s| <= 1e5), lowercase English letters

use std::{
    io::{self, BufRead},
};

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).unwrap();

    let s = input.trim();

    let mut freq = [0;26];
    let a: u8 = 97;
    for ch in s.chars() {
        let ascii_ch = ch as u8;
        freq[(ascii_ch - a) as usize] += 1;
    }

    let mut i =0;

    while i<26 {
        if freq[i] > 0{
            println!("{}:{}", (a+i as u8) as char, freq[i])
        }
        i+=1;
    }
}
