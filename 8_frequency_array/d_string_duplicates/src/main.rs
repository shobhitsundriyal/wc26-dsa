// D. String Duplicates
// https://codeforces.com/group/DNruwPkRNA/contest/728252/problem/D
//
// Input
//   Line 1: s (1 <= |s| <= 1e6), lowercase English letters

use std::{io::{self, BufRead}};

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let s = input.trim();

    let mut freq = vec![0;26];
    let a: u8 = 97;
    for ch in s.chars() {
        let ascii_ch = ch as u8;
        freq[(ascii_ch - a) as usize] += 1;
    }

    let mut flag = false;
    let mut i =0;

    while i<26 {
        if freq[i] > 1{
            flag =true;
            print!("{}", (a+i as u8) as char)
        }
        i+=1;
    }

    if !flag{
        print!("-1");
    }
}
