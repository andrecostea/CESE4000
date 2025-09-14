struct Range {
    start: usize,
    end: usize,
}

impl Range {
    /// Create a new range, panics if `end <= start`
    fn new(start: usize, end: usize) -> Self {
        assert!(end > start, "end must be greater than start");
        Range { start, end }
    }

    // Length of the range
    fn len(&self) -> usize {
        self.end - self.start
    }

    // Middle of the range
    fn middle(&self) -> usize {
        (self.start + self.end) / 2
    }
}

fn main() {
    let r = Range::new(10, 20);
    println!("Length: {}", r.len());    // 10
    println!("Middle: {}", r.middle()); // 15

    let r2: Range = Range{start: 2,end: 3};

    // Uncommenting this will panic:
    // let bad = Range::new(5, 2);
}
