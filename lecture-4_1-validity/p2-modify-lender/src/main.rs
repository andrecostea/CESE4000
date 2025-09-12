fn main(){  
   let mut x = vec![1, 2, 3];         

   let i = &x;

   // x = vec![3,4];  // this binding, if allowed, could cause an UF in the next dereference
 
   println!("{}", i[0]);
 }