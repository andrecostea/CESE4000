fn main() {
  let mut x: Vec<i32> = vec![1, 2, 3]; 

  x.push(4);

  let first_elem: &i32 = &x[0];                           

  // x.push(4);          

  println!("{}", first_elem);          
}                                      