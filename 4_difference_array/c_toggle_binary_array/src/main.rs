// C. Toggle Binary Array
// https://codeforces.com/group/DNruwPkRNA/contest/726822/problem/C

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
        queries.push((a-1, b-1));
    }

    let mut diff_arr = vec![0;n];
    queries.iter().for_each(|(a, b)| {
        diff_arr[*a] += 1;
        if *b+1 != n {
            diff_arr[*b+1] -= 1;
        }
    });
    //convert to prefix sum
    let mut i =1;
    let first_elment= if diff_arr[0]%2==0 {
        arr[0]
    } else {
        arr[0]^1
    };
    let mut final_arr: Vec<i64> = vec![first_elment];
    while i<n {
        diff_arr[i] += diff_arr[i-1];
        if diff_arr[i]%2 == 0 {
            final_arr.push(arr[i]);
        } else {
            final_arr.push(arr[i]^1); //xor will toggle 
        }
        i+=1;
    }

    // println!("{:?}", diff_arr);
    // println!("{:?}", final_arr);
    final_arr.iter().for_each(|element| print!("{element} "));
}
