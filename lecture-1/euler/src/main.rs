const A: u32 = 3;
const B: u32 = 5;
const LIMIT: u32 = 1000;

/// Compute the sum of all multiples of `a` or `b` below `limit` using a loop.
fn sum_of_multiples(a: u32, b: u32, limit: u32) -> u32 {
    let mut sum = 0;
    for n in 1..limit {
        if n % a == 0 || n % b == 0 {
            sum += n;
        }
    }
    sum
}

fn main() {
    let result = sum_of_multiples(A, B, LIMIT);
    println!(
        "The sum of all multiples of {} or {} below {} is {}.",
        A, B, LIMIT, result
    );
}