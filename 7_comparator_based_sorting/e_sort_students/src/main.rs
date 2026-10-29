// E. Sort Students
// https://codeforces.com/group/DNruwPkRNA/contest/727651/problem/E

use std::{io::{self, BufRead}};

fn main() {
    let stdin = io::stdin();

    // Read tokens lazily instead of slurping all of stdin, so the program
    // stops as soon as the needed input is consumed instead of waiting for EOF.
    let mut tokens = stdin
        .lock()
        .lines()
        .flat_map(|line| line.unwrap().split_whitespace().map(String::from).collect::<Vec<_>>());

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut students: Vec<(String, i64)> = (0..n)
        .map(|_| {
            let name = tokens.next().unwrap();
            let marks = tokens.next().unwrap().parse().unwrap();
            (name, marks)
        })
        .collect();

        students.sort_by(|s1, s2| {
            if s1.1 == s2.1 {                
                s1.0.cmp(&s2.0)
            } else {
                s2.1.cmp(&s1.1)
            }
        });

        students.iter().for_each(|s| println!("{} {}", s.0, s.1));
}
