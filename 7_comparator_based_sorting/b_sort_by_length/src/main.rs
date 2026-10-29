// B. Sort by length
// https://codeforces.com/group/DNruwPkRNA/contest/727651/problem/B

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    // Read tokens lazily instead of slurping all of stdin, so the program
    // stops as soon as the needed input is consumed instead of waiting for EOF.
    let mut tokens = stdin
        .lock()
        .lines()
        .flat_map(|line| line.unwrap().split_whitespace().map(String::from).collect::<Vec<_>>());

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut strings: Vec<String> = (0..n).map(|_| tokens.next().unwrap()).collect();

    strings.sort_by(|x,y| {
        if x.len() < y.len() {
            std::cmp::Ordering::Less
        } else if x.len() > y.len() {
            std::cmp::Ordering::Greater
        } else {
            x.cmp(y)
        }
    });

    strings.iter().for_each(|s| println!("{s}"));
}
