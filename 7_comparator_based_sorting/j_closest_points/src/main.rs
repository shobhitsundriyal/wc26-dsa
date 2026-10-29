// J. Closest Points
// https://codeforces.com/group/DNruwPkRNA/contest/727651/problem/J

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
    let a: i64 = tokens.next().unwrap().parse().unwrap();
    let b: i64 = tokens.next().unwrap().parse().unwrap();
    let mut points: Vec<(i64, i64)> = (0..n)
        .map(|_| {
            let x = tokens.next().unwrap().parse().unwrap();
            let y = tokens.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();

    points.sort_by(|p1,p2| {
        let d1 = (((p1.0 - a).pow(2) + (p1.1 - b).pow(2)) as f64).sqrt();
        let d2 = (((p2.0 - a).pow(2) + (p2.1 - b).pow(2)) as f64).sqrt();

        let is_x_same = p1.0 == p2.0;
        let is_d_same = d1 == d2;

        if is_x_same && is_d_same {
            p1.1.cmp(&p2.1)
        } else if is_d_same {
            p1.0.cmp(&p2.0)
        } else {
            if d1<d2 {std::cmp::Ordering::Less} else { std::cmp::Ordering::Greater}
        }
    });
    points.iter().for_each(|s| println!("{} {} ", s.0, s.1));

}
