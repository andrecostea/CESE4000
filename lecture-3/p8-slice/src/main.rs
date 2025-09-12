fn main(){
  let mut x = vec![1, 2, 3, 4];

  // index 0, and 1 (excluding 2)
  let a: &mut [u32] = &mut x[0..2];
  for i in a {
    *i += 3;
  }

  // prints 4, 5, 3, 4
  println!("{:?}", x);
}