// F. Sort Points
// https://codeforces.com/group/DNruwPkRNA/contest/727651/problem/F

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
    let mut points: Vec<(i64, i64)> = (0..n)
        .map(|_| {
            let x = tokens.next().unwrap().parse().unwrap();
            let y = tokens.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();

    points.sort_by(|s1, s2| {
            if s1.0 == s2.0 {                
                s1.1.cmp(&s2.1)
            } else {
                s1.0.cmp(&s2.0)
            }
        });

        points.iter().for_each(|s| println!("{} {}", s.0, s.1));
}
