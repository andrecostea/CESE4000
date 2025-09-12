  fn fill_vector_with_ref_1(src: &u32, dst: &mut Vec<u32>) {
    for i in 0..dst.len() { dst[i] = *src + 1; }
  }

  fn fill_vector_with_ref_2(src: &u32, dst: &mut Vec<u32>) {
    let value = *src + 1;
    for i in 0..dst.len() { dst[i] = value; }
  }

fn main(){
  let mut x = vec![1, 2, 3];
  let y = 0;
  // obviously wrong
  // mutable *and* immutable reference at the same time
  // fill_vector_with_ref_1(&x[1], &mut x);
  fill_vector_with_ref_1(&y, &mut x);
  println!("{x:?}");
}