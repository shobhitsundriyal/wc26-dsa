use std::{io::{self, BufRead}};

fn natural_sum(n: u64) -> u64 {
    n * (n + 1) / 2
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let q: usize = lines
        .next()
        .expect("missing Q")
        .expect("failed to read line")
        .trim()
        .parse()
        .expect("Q");
    
    let mut ans: Vec<u64> = vec![];

    for _ in 0..q {
        let line = lines
            .next()
            .expect("missing query line")
            .expect("failed to read line");
        let mut tokens = line.split_whitespace();

        let l: u64 = tokens.next().expect("missing L").parse().expect("L");
        let r: u64 = tokens.next().expect("missing R").parse().expect("R");

        // your logic here


        ans.push(natural_sum(r) - natural_sum(l) + l); 
    }

    ans.iter().for_each(|element| println!("{element}"));
}
