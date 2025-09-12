fn main(){  
   let mut x = vec![1, 2, 3];         

   let i = &x;

   // let y = x;  // this move invalidates the subsequent uses of the imm reference
 
   println!("{}", i[0]);
 }