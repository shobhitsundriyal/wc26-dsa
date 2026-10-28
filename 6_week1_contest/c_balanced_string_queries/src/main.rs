// C. Balanced String Queries
// https://codeforces.com/group/DNruwPkRNA/contest/727638/problem/C
//
// Input (matches the provided template)
//   Line 1: n q (1 <= n, q <= 1e5)
//   Line 2: s, a string of length n containing only 0 and 1
//   Next q lines: L R (1 <= L <= R <= n), 1-based and inclusive
//
// Output
//   One line per query: "YES" if s[L..=R] has equal zeros and ones,
//   otherwise "NO".

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    // Read tokens lazily instead of slurping all of stdin, so the program
    // stops after the q queries instead of waiting for EOF.
    let mut tokens = stdin
        .lock()
        .lines()
        .flat_map(|line| line.unwrap().split_whitespace().map(String::from).collect::<Vec<_>>());

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let q: usize = tokens.next().unwrap().parse().unwrap();

    // Line 2 is a single string of '0'/'1' chars; store it as a Vec of digit
    // values so s[i] can be compared with 0 and 1 directly.
    let s: Vec<usize> = tokens
        .next()
        .unwrap()
        .bytes()
        .map(|b| (b - b'0') as usize)
        .collect();
    debug_assert_eq!(s.len(), n);

    // Queries stay 1-based, as numbered in the statement.
    let queries: Vec<(usize, usize)> = (0..q)
        .map(|_| {
            let l: usize = tokens.next().unwrap().parse().unwrap();
            let r: usize = tokens.next().unwrap().parse().unwrap();
            (l, r)
        })
        .collect();

    // TODO: solve
    let mut p_arr0 = vec![0];
    let mut p_arr1 = vec![0];
    let mut i = 1;
    
    while i<=n {
        p_arr0.push(p_arr0[i-1] + if s[i-1]==0 {1} else {0});
        p_arr1.push(p_arr1[i-1] + if s[i-1]==1 {1} else {0});
        
        i+=1;
    }
    // println!("{:?}", p_arr0);
    queries.iter().for_each(|(l, r)| {
        let sum0 = if *l==1 {p_arr0[*r]} else {p_arr0[*r] - p_arr0[*l-1]};
        let sum1 = if *l==1 {p_arr1[*r]} else {p_arr1[*r] - p_arr1[*l-1]};
        if sum0 == sum1 {
            println!("YES");
        } else {
            println!("NO");
            
        }
    });
}
