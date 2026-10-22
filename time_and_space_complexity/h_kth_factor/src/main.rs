use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let line = lines
        .next()
        .expect("missing N and K")
        .expect("failed to read line");
    let mut tokens = line.split_whitespace();

    let n: u64 = tokens.next().expect("missing N").parse().expect("N");
    let k: u64 = tokens.next().expect("missing K").parse().expect("K");

    // your logic here

    // if n==1 {println!("1");exit(0);}
    
    let sqrt = (n as f64).sqrt();
    let mut i = 1.0;
    let mut half_factors: Vec<u64> = vec![];
    while i <= sqrt {
        if n.is_multiple_of(i as u64) {
            half_factors.push(i as u64);
        }
        i += 1.0;
    }
    let mut other_half: Vec<u64>= vec![];
    let mut j = half_factors.len() as i64 - 1;
    let mut flag = 0;
    while j>=0 {
        if flag==0 && n / half_factors[j as usize] == half_factors[j as usize] {
            flag = 1;
        } else {
            other_half.push(n/half_factors[j as usize]);
        }
        j -= 1;
    }
    half_factors.extend(other_half);

    if k > half_factors.len() as u64 {
        println!("-1");
    } else {
        println!("{}", half_factors[k as usize -1]);
    }

}
