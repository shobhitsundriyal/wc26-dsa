use std::{io::{self, BufRead}, process::exit};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: u64 = lines
        .next()
        .expect("missing N")
        .expect("failed to read line")
        .trim()
        .parse()
        .expect("N");

    // your logic here

    if n==1 {println!("1");exit(0);}
    
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
    half_factors.iter().for_each(|element| print!("{element} "));
    other_half.iter().for_each(|element| print!("{element} "));
    // half_factors.iter().for_each(|element| print!("{element} "));
    // half_factors.reverse();
    // half_factors.iter().for_each(|element| print!("{} ", n / element));
}
