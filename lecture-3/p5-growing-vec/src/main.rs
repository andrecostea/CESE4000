 fn main(){ 
  let mut x = vec![1, 2, 3];
  
  // first reference, to an element
  let first_elem = &x[0];
  // second reference, mutable this time
  // x.push(4);      // COMPILATION ERROR: cannot take a mutable borrow while an immutable one exist. Possible implications: the vector is reallocated elswhere on the heap

  println!("{}", first_elem);
}