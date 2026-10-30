// L. Odd Occurence Queries
// https://codeforces.com/group/DNruwPkRNA/contest/728252/problem/L
//
// Input
//   Line 1: q (1 <= q <= 1e6)
//   Next q lines: one of
//     "1 x"  add x
//     "2 x"  remove one occurrence of x (if present)
//     "3"    print how many distinct numbers appear an odd number of times
//   (1 <= x <= 1e5)

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let q: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut freq = [0;100001];
    let mut ans: Vec<u32> = vec![];
    let mut count = 0;

    for _ in 0..q {
        let line = lines.next().unwrap().unwrap();
        let mut tokens = line.split_whitespace();

        let kind: u8 = tokens.next().unwrap().parse().unwrap();

        match kind {
            1 | 2 => {
                let x: usize = tokens.next().unwrap().parse().unwrap();
                if kind ==1 {
                    freq[x] += 1;
                    if freq[x] %2 != 0 {
                        count += 1;
                    } else {
                        count -= 1;
                    }
                } else {
                    if freq[x] > 0 {

                        freq[x] -= 1;
                        if freq[x] %2 != 0 {
                            count += 1;
                        } else {
                            count -= 1;
                        }
                    }
                }
                
            }
            3 => {
                // let x: usize = tokens.next().unwrap().parse().unwrap();
                ans.push(count);
            }
            _ => unreachable!("unknown query type: {kind}"),
        }
    }
    ans.iter().for_each(|e| println!("{e}"));
}
