fn length(v: Vec<i32>) -> usize { 
  v.len() 
} // v---the new owner of the vector---goes out of scope so the vector is deallocated here
fn main(){
  let x = vec![1, 2, 3];                 
  let y = length(x);        // MOVE inside the function call
  println!("The length of the vector is {y:?}");  
}