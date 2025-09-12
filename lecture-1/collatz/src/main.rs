 // To denote constants, the convention is to use ALL_CAPS (with underscores if needed) 
 
const LIMIT: u64 = 1_000_000;

/// Compute the length of the Collatz sequence starting from `n`.
fn collatz_length(mut n: u64) -> u64 {
    let mut length = 1;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        length += 1;
    }
    length
}

/// Find the starting number under LIMIT with the longest Collatz sequence.
fn main() {
    let mut max_length = 0;
    let mut number_with_max = 0;

    let mut n = 1;
    while n < LIMIT {
        let length = collatz_length(n);
        if length > max_length {
            max_length = length;
            number_with_max = n;
        }
        n += 1;
    }

    println!(
        "The starting number under {} that produces the longest Collatz chain is {} (length {}).",
        LIMIT, number_with_max, max_length
    );
}
