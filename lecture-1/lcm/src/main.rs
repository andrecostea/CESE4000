const START: u32 = 1;
const END: u32 = 20;

/// Compute the greatest common divisor (GCD) using Euclid's algorithm.
fn gcd(a: u32, b: u32) -> u32 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}

/// Compute the least common multiple (LCM) of two numbers.
fn lcm(a: u32, b: u32) -> u32 {
    a / gcd(a, b) * b
}

/// Compute the LCM of all numbers in the range [start, end].
fn lcm_range(start: u32, end: u32) -> u32 {
    let mut result = start;
    let mut n = start + 1;
    while n <= end {
        result = lcm(result, n);
        n += 1;
    }
    result
}

fn main() {
    let result = lcm_range(START, END);
    println!(
        "The smallest positive number divisible by all numbers from {} to {} is {}.",
        START, END, result
    );
}
