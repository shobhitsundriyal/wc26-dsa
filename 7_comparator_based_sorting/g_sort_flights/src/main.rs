// G. Sort Flights
// https://codeforces.com/group/DNruwPkRNA/contest/727651/problem/G

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    // Read tokens lazily instead of slurping all of stdin, so the program
    // stops as soon as the needed input is consumed instead of waiting for EOF.
    let mut tokens = stdin
        .lock()
        .lines()
        .flat_map(|line| line.unwrap().split_whitespace().map(String::from).collect::<Vec<_>>());

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut flights: Vec<(i64, i64, String)> = (0..n)
        .map(|_| {
            let flight_number = tokens.next().unwrap().parse().unwrap();
            let departure_time = tokens.next().unwrap().parse().unwrap();
            let destination = tokens.next().unwrap();
            (flight_number, departure_time, destination)
        })
        .collect();

    flights.sort_by(|s1, s2| {
            if s1.1 == s2.1 {                
                s1.0.cmp(&s2.0)
            } else {
                s1.1.cmp(&s2.1)
            }
        });

    flights.iter().for_each(|s| println!("{} {} {}", s.0, s.1, s.2));
}
