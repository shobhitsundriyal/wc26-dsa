use std::{io::{self, BufRead}, process::exit};

fn is_prime(n: u64) -> bool {
    let sqrt = (n as f64).sqrt();
    let mut i = 2.0;
    let mut count = 2;
    while i <= sqrt {
        if n.is_multiple_of(i as u64) {
            if i == sqrt {
                count += 1;
            } else {
                count += 2;
            }
        }
        i += 1.0;
    }
    count == 2
}

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

    if n==1 {println!("0");exit(0);}

    let mut i: u64 = 2;
    let mut count_primes = 0;
    while i <= n {
        if is_prime(i) {
            count_primes += 1;
        }
        
        i+=1;
    }
    println!("{count_primes}")
    
}
