fn main(){  
   let y = vec![1, 2, 3];         

   let mut x = y;   

   // swapping lines 7 and 9 would lead to a compilation error. Why?
   x.push(4);

   let i = &x[0];
 
   println!("{}", i);
 }