// H. Sort Stocks
// https://codeforces.com/group/DNruwPkRNA/contest/727651/problem/H

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
    let mut stocks: Vec<(String, i64, i64)> = (0..n)
        .map(|_| {
            let stock_name = tokens.next().unwrap();
            let price = tokens.next().unwrap().parse().unwrap();
            let market_cap = tokens.next().unwrap().parse().unwrap();
            (stock_name, price, market_cap)
        })
        .collect();

    stocks.sort_by(|s1, s2 | {
        let is_price_same = s1.1 == s2.1;
        let is_cap_same = s1.2 == s2.2;

        if is_cap_same && is_price_same {
            s1.0.cmp(&s2.0)
        } else if is_price_same {
            s2.2.cmp(&s1.2)
        } else {
            s1.1.cmp(&s2.1)
        }
    });

    stocks.iter().for_each(|s| println!("{} {} {}", s.0, s.1, s.2));
}
