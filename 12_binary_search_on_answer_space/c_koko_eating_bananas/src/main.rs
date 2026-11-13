// C. Koko Eating Bananas
// https://codeforces.com/group/DNruwPkRNA/contest/730116/problem/C
//
// Input
//   Line 1: n h (1 <= n <= 2e5, 1 <= h <= 1e18)
//   Line 2: n integers a1 .. an (1 <= ai <= 1e9)

use std::io::{self, BufRead};

fn is_possible(arr: &[u64], check_rate: u64, hours_allowed: u64) -> bool {
    let mut hr_taken = 0;

    for pile in arr {
        // if *pile <= check_rate {
        //     hr_taken += 1;
        // } else {
        //     let rem = pile % check_rate;
        //     let div = pile / check_rate;
        //     hr_taken += if rem == 0 {div} else {div+1};
        // }
        //or better way
        if pile % check_rate == 0 {
            hr_taken += pile/check_rate;
        } else {
            hr_taken += (pile/check_rate) + 1;
        }
    }

    hr_taken <= hours_allowed
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Line 1: n h
    let first = lines.next().expect("missing first line").unwrap();
    let mut first = first.split_whitespace();

    let n: usize = first.next().unwrap().parse().unwrap();
    let h: u64 = first.next().unwrap().parse().unwrap();

    // Line 2 (and any following lines, in case it wraps): the n piles.
    let mut a: Vec<u64> = Vec::with_capacity(n);
    while a.len() < n {
        let line = lines.next().expect("not enough piles").unwrap();
        a.extend(
            line.split_whitespace()
                .map(|t| t.parse::<u64>().unwrap()),
        );
    }

    let mut l = 1;
    let mut r = *a.iter().max().unwrap();
    let mut ans = r;

    while l<=r {
        let mid = (l+r)/2;
        if is_possible(&a,mid, h) {
            r = mid -1;
            ans = mid;
        } else {
            l = mid +1;
        }
    }

    println!("{ans}");
}
