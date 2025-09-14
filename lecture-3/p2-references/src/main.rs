fn main() {
  let mut x = vec![1, 2, 3];

  let first_elem = &x[0];

  // x.push(4);  //COMPILATION ERROR: while a shared reference is alive, we cannot modify the lender

  println!("{}", first_elem);
}
