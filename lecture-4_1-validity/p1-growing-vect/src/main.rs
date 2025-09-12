fn main(){  
   let y = vec![1, 2, 3];         

   let mut x = y;   

   x.push(4);

   let i = &x[0];
 
   println!("{}", i);
 }