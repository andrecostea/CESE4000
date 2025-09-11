enum List {
    End,
    Item(u32, Box<List>),
}

impl List {
    /// Build a list from a slice of numbers
    fn create(items: &[u32]) -> List {
        // Build backwards: start from the end
        let mut result = List::End;
        for &x in items.iter().rev() {
            result = List::Item(x, Box::new(result));
        }
        result
    }

    /// Compute the length of the list
    fn length(&self) -> usize {
        match self {
            List::End => 0,
            List::Item(_, next) => 1 + next.length(),
        }
    }
}

fn main() {
    let nums = [10, 20, 30];
    let list = List::create(&nums);

    println!("Length: {}", list.length()); // 3
}