
// Immutable slices can co-exist
fn main(){ 
  let x = vec![1, 2, 3, 4];
  
  let a: &[u32] = &x[0..2];   // index 0, and 1 (excluding 2)
  let b = &x[2..];    // elements at indexes starting from 2

  for i in b {          // you can iterate over a slice
    println!("{i}");          // 3, 4 
  }

  println!("{}", a.len());    // or get its length, i.e. 2
 }