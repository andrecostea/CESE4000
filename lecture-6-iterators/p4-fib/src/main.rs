struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Self { a: 0, b: 1 } // initiate from the first two numbers
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.a;
        self.a = self.b;
        self.b = next + self.b;
        Some(next)
    }
}

fn main() {
    let fib = Fibonacci::new();

    for x in fib.take(10) { // take first 10 numbers
        println!("{}", x);
    }
}

