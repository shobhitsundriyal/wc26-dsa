// C. Check Anagram
// https://codeforces.com/group/DNruwPkRNA/contest/728252/problem/C
//
// Input
//   Line 1: s (1 <= |s| <= 1e5), lowercase English letters
//   Line 2: t (1 <= |t| <= 1e5), lowercase English letters

use std::{
    collections::HashMap, io::{self, BufRead}, process::exit,
};

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let s = input.trim();

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let t = input.trim();

    let mut map: HashMap<char, i64> = HashMap::new();

    for ch in s.chars() {
        map.insert(ch, map.get(&ch).unwrap_or(&0) + 1);
    }
    for ch in t.chars() {
        map.insert(ch, map.get(&ch).unwrap_or(&0) - 1);
    }
    // let ans = "NO";
    for (_,v) in &map {
        if *v != 0 {
            print!("NO");
            exit(0);
        }
    }
    print!("YES");
}
