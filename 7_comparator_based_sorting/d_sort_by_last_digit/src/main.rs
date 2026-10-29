// D. Sort by last digit
// https://codeforces.com/group/DNruwPkRNA/contest/727651/problem/D

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
    let mut a: Vec<i64> = (0..n)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();

    a.sort_by(|x,y| {
        let x_a = (x%10).abs();
        let y_a = (y%10).abs();
        if x_a == y_a {
            x.cmp(y)
        } else {
            x_a.cmp(&y_a)
        }
    });
    a.iter().for_each(|e| print!("{e} "));
}
