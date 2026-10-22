/// Sum of the first `n` natural numbers.
///
/// Loop: O(n) time, O(1) space.
/// Formula: O(1) time, O(1) space.
#[allow(dead_code)]
fn natural_sum_loop(n: u64) -> u64 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

fn natural_sum_formula(n: u64) -> u64 {
    n * (n + 1) / 2
}

fn read_n() -> u64 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read stdin");

    input
        .trim()
        .parse()
        .expect("expected a non-negative integer")
}

fn main() {
    let n = read_n();

    // println!("n = {n}");
    // println!("loop    -> {}", natural_sum_loop(n));
    println!("{}", natural_sum_formula(n));
}
