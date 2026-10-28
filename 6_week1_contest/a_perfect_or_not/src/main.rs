// A. Perfect or Not
// https://codeforces.com/group/DNruwPkRNA/contest/727638/problem/A
//
// Input (matches the provided template)
//   Line 1: t, the number of values to classify (at most 1000)
//   Next t lines: n (1 <= n <= 1e6)
//
// Output
//   One line per value: "Perfect", "Abundant" or "Deficient".
//   (The official template calls classify(n) t times.)

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    // Read tokens lazily instead of slurping all of stdin, so the program
    // stops after the t values instead of waiting for EOF.
    let mut tokens = stdin
        .lock()
        .lines()
        .flat_map(|line| line.unwrap().split_whitespace().map(String::from).collect::<Vec<_>>());

    let t: usize = tokens.next().unwrap().parse().unwrap();

    let ns: Vec<f32> = (0..t)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();

    // TODO: solve
    ns.iter().for_each(|n| {
        let mut sum =0.0;
        let mut i = 1.0;
        let sqrt = n.sqrt();
        while i<=sqrt {
            if n%i == 0.0 {
                if i*i == *n {
                    sum += i;
                } else {
                    sum += i + n/i;
                }
            }
            i+=1.0;
        }
        sum -= *n;
        if sum > *n {
            println!("Abundant");
        } else if sum < *n {
            println!("Deficient");
        } else {
            println!("Perfect");
        }
    });
}
