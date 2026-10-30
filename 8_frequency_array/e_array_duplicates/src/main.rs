// E. Array Duplicates
// https://codeforces.com/group/DNruwPkRNA/contest/728252/problem/E
//
// Input
//   Line 1: n (1 <= n <= 1e6)
//   Line 2: n integers a1 .. an (1 <= ai <= 1e5)

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut a: Vec<usize> = Vec::with_capacity(n);
    while a.len() < n {
        let line = lines.next().unwrap().unwrap();
        for tok in line.split_whitespace() {
            a.push(tok.parse().unwrap());
        }
    }

    a.sort();
    a.push(a[n - 1] + 1);

    let mut i = 1;
    let mut pre = a[0];
    let mut count = 1;
    let mut flag = false;
    while i <= n {
        if pre != a[i] {
            if count > 1 {
                print!("{pre} ");
                flag = true;
            }
            pre = a[i];
            count = 1;
        } else {
            count += 1;
        }
        i+=1;
    }
    if !flag {
        print!("-1")
    }
}
