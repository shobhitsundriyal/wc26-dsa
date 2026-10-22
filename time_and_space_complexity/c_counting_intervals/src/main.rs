use std::{io::{self, BufRead}};
fn max_of_zero(n: i64) -> i64 {
    if n > 0 {
        return n;
    }
    return 0;
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

    let mut ans:Vec<i64> = vec![];

    for _ in 0..q {
        let line = lines
            .next()
            .expect("missing query line")
            .expect("failed to read line");
        let mut tokens = line.split_whitespace();

        let t: i64 = tokens.next().expect("missing T").parse().expect("T");
        let l: i64 = tokens.next().expect("missing L").parse().expect("L");
        let r: i64 = tokens.next().expect("missing R").parse().expect("R");

        // your logic here
        match t {
            1 => {ans.push(max_of_zero(r-l-1));}
            2 | 3 => {ans.push(max_of_zero(r-l));}
            4 => {ans.push(max_of_zero(r-l+1));}
            _ => {}
        }
    }

    ans.iter().for_each(|element| println!("{}", element));
}
