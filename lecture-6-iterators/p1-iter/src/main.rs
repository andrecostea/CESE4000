fn main() {
    let v = vec![1,2,3];
    // let mut v = Vec::new();
    // v.push(1); v.push(2); v.push(3);
    let w = v.iter().map(|_| 0);
    println!("{:?}",w);
}
