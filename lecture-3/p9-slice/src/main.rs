// input is a slice
  fn sum(res: &[u32]) -> u32 {  res.iter().sum() }

  fn sum_rec(input: &[u32]) -> u32 {
    if input.is_empty() {
      0
    } else {
      // add element 0 to everything after element 0
      input[0] + sum_rec(&input[1..])
    }
  }

fn main(){
  // but we can call it with a vector!
  let x = vec![1, 2, 3];
  sum(&x);
  // or a bit of a vector
  sum(&x[1..]);
  // or an array
  sum(&[1, 2, 3]);
}