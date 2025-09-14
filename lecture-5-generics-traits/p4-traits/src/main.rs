// Define the trait
trait ToString {  /* --------- Q:  What happens if I delete this trait definition? --------- */
    fn to_string(&self) -> String;
} 

// A wrapped character
struct ID(u32);

// Implement the trait
impl ToString for ID {
    fn to_string(&self) -> String {
        format!("id_{}", self.0)  
    }
}

// A point in 2D space
struct Point {
    x: f64,
    y: f64,
}

impl ToString for Point {
    fn to_string(&self) -> String {
        format!("Point({}, {})", self.x, self.y)
    }
}

// This function works on anything that implements ToString (notice the `impl` annotation in the function signature)
fn print_string(item: &impl ToString) {
    println!("Item: {}", item.to_string());
}

fn main() {
    let n = ID(0);
    let p = Point { x: 1.0, y: 2.0 };

    print_string(&n);
    print_string(&p);
    // print_string(&1); // COMPILATION ERROR: does not implement ToString
}
