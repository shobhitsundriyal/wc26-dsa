// B. Array Manipulation II
// https://codeforces.com/group/DNruwPkRNA/contest/726822/problem/B

use std::io::{self, BufRead};

fn main() {
    // Read lazily, line by line, so we stop as soon as all input is read
    // instead of waiting for EOF like read_to_string does.
    let stdin = io::stdin();
    let mut tokens = stdin
        .lock()
        .lines()
        .flat_map(|line| line.unwrap().split_whitespace().map(String::from).collect::<Vec<_>>());

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let q: usize = tokens.next().unwrap().parse().unwrap();

    // Initial array values (1-indexed conceptually; stored 0-indexed).
    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        let x: i64 = tokens.next().unwrap().parse().unwrap();
        arr.push(x);
    }

    let mut queries = Vec::with_capacity(q);
    for _ in 0..q {
        let a: usize = tokens.next().unwrap().parse().unwrap();
        let b: usize = tokens.next().unwrap().parse().unwrap();
        let k: i64 = tokens.next().unwrap().parse().unwrap();
        queries.push((a-1, b-1, k));
    }

    let mut diff_arr = vec![0;n];
    queries.iter().for_each(|(a, b, k)| {
        diff_arr[*a] += k;
        if *b+1 != n {
            diff_arr[*b+1] -= k;
        }
    });
    //convert to prefix sum
    let mut i =1;
    let mut final_arr: Vec<i64> = vec![arr[0] + diff_arr[0]];
    while i<n {
        diff_arr[i] += diff_arr[i-1];
        final_arr.push(arr[i] + diff_arr[i]);
        i+=1;
    }

    let ans = final_arr.iter().max().unwrap();
    println!("{ans}");
}
