// I. Sort Amazon Orders
// https://codeforces.com/group/DNruwPkRNA/contest/727651/problem/I

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
    let mut orders: Vec<(i64, i64, i64)> = (0..n)
        .map(|_| {
            let order_id = tokens.next().unwrap().parse().unwrap();
            let timestamp = tokens.next().unwrap().parse().unwrap();
            let priority = tokens.next().unwrap().parse().unwrap();
            (order_id, timestamp, priority)
        })
        .collect();

    orders.sort_by(|s1, s2 | {
        let is_priority_same = s1.2 == s2.2;
        let is_time_same = s1.1 == s2.1;

        if is_time_same && is_priority_same {
            s1.0.cmp(&s2.0)
        } else if is_priority_same {
            s1.1.cmp(&s2.1)
        } else {
            s2.2.cmp(&s1.2)
        }
    });

    orders.iter().for_each(|s| println!("{} {} {}", s.0, s.1, s.2));
}
