// A. Array Manipulation
// https://codeforces.com/group/DNruwPkRNA/contest/726822/problem/A

use std::io::{self, BufRead};

fn main() {
    // Read lazily, line by line, so we stop as soon as q queries are read
    // instead of waiting for EOF like read_to_string does.
    let stdin = io::stdin();
    let mut tokens = stdin
        .lock()
        .lines()
        .flat_map(|line| line.unwrap().split_whitespace().map(String::from).collect::<Vec<_>>());

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let q: usize = tokens.next().unwrap().parse().unwrap();

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
    //create prefix sum
    let mut p_arr = vec![diff_arr[0]];
    let mut i =1;
    while i<n {
        p_arr.push(p_arr[i-1] + diff_arr[i]);
        i+=1;
    }
    let max = p_arr.iter().max();
    println!("{}", max.unwrap());
}
