// B. Count Vowels
// https://codeforces.com/group/DNruwPkRNA/contest/698784/problem/B
//
// Input
//   Line 1: n k (1 <= k <= n <= 1e5)
//   Line 2: s (uppercase and lowercase English letters, length n)

use std::io::{self, BufRead};

fn is_vowel(ch: char) -> bool {
    matches!(ch.to_ascii_lowercase(), 'a'|'e'|'i'|'o'|'u')
}

fn main() {
    let stdin = io::stdin();
    let mut tokens = stdin
        .lock()
        .lines()
        .flat_map(|line| line.unwrap().split_whitespace().map(str::to_string).collect::<Vec<_>>());

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let k: usize = tokens.next().unwrap().parse().unwrap();

    let s: String = tokens.next().unwrap().to_string();

    let mut i=0;
    let mut contri = vec![0; n];
    // create conti arr, would 2N time but chalega
    for (idx, ch) in s.chars().enumerate() {
        if is_vowel(ch) {
            contri[idx] = 1;
        }
    }

    let mut count =0;
    while i<k {
        count += contri[i];
        i+=1;
    }
    print!("{count} ");

    while i<n {
        count += contri[i] - contri[i-k];

        print!("{count} ");
        i+=1;
    }
}
