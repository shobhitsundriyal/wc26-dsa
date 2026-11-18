// L. Minimum Element
// https://codeforces.com/group/DNruwPkRNA/contest/733421/problem/L
//
// Input
//   Line 1: n (1 <= n <= 1e5)
//   Line 2: n integers a1 .. an (-1e9 <= ai <= 1e9)

use std::io::{self, BufRead};

fn mini(a: &[i64], n: usize, curr_ans: i64) -> i64 {
    if n==0 {
        return  curr_ans;
    }
    if curr_ans > a[n] {
        mini(a, n-1, a[n])
    } else {
        mini(a, n-1, curr_ans)
    }

}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n
    let first = lines.next().expect("missing first line").unwrap();
    let n: usize = first.trim().parse().unwrap();

    // Line 2 (and any following lines, in case it wraps): the n values.
    let mut a: Vec<i64> = Vec::with_capacity(n+1);
    a.push(0);
    while a.len() < n+1 {
        let line = lines.next().expect("not enough array values").unwrap();
        a.extend(
            line.split_whitespace()
                .map(|t| t.parse::<i64>().unwrap()),
        );
    }

    println!("{}", mini(&a, n, a[n]))
}
