// H. Unique Queries
// https://codeforces.com/group/DNruwPkRNA/contest/728252/problem/H
//
// Input
//   Line 1: q (1 <= q <= 1e6)
//   Next q lines: one of
//     "+ x"  insert x
//     "- x"  remove one occurrence of x (if present)
//     "?"    count unique elements
//   (1 <= x <= 1e5)

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let q: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut freq = [0;100001];
    let mut ans: Vec<u32> = vec![];
    let mut distict_count = 0;
    for _ in 0..q {
        let line = lines.next().unwrap().unwrap();
        let mut tokens = line.split_whitespace();

        let op = tokens.next().unwrap();

        match op {
            "?" => {
                // TODO: print the number of unique elements
                ans.push(distict_count);
            }
            "+" | "-" => {
                let x: usize = tokens.next().unwrap().parse().unwrap();
                let _ = x;

                // TODO: insert / remove x
                if op == "+" {
                    freq[x] += 1;
                    if freq[x] == 1 {
                        distict_count += 1;
                    }
                }
                if op == "-" {
                    if freq[x] == 1 {
                        distict_count -= 1;
                    }
                    freq[x] -= if freq[x] > 0 {1} else {0};
                }
            }
            _ => unreachable!("unknown query type: {op}"),
        }
    }
    ans.iter().for_each(|e| println!("{e}"));
}
