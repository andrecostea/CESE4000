  // The two functions below have the same functionality, assuming not aliasing.
  fn fill_vector_with_ref_1(src: &u32, dst: &mut Vec<u32>) {
    for i in 0..dst.len() { dst[i] = *src + 1; }
  }

  // The compiler could make use of the non-alias info to optimise the above function into the below one
  fn fill_vector_with_ref_2(src: &u32, dst: &mut Vec<u32>) {
    let value = *src + 1;
    for i in 0..dst.len() { dst[i] = value; }
  }

fn main(){
  let mut x = vec![1, 2, 3];
  let y = 0;
  // obviously wrong
  // mutable *and* immutable reference cannot live at the same time
  // fill_vector_with_ref_1(&x[1], &mut x); // COMPILATION ERROR

  // This is ok, y and x point to non-overlapping locations
  fill_vector_with_ref_1(&y, &mut x);
  println!("{x:?}");
}