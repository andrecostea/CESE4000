fn main() {
    // let data = [1, 2, 3, 4, 5];
    let data: [u32; 100] = std::array::from_fn(|i| (i + 1) as u32);

    // Create an iterator pipeline
    let iter = data
        .iter()
        .map(|x| {
            println!("Mapping {}", x);
            x * x
        })
        .filter(|x| {
            println!("Filtering {}", x);
            x % 4 == 0
        });

    println!("--- Before collecting ---");

    // Nothing has happened yet!
    //let result: Vec<_> = iter.collect();
    let result: Vec<_> = iter.take(5).collect();  // collect forces the iterator to consume the elements

    println!("--- After collecting ---");
    // println!("Data: {:?}", data);
    println!("Result: {:?}", result);
}