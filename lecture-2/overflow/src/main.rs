// comment this line out to force compiler check for overflows 
#[allow(arithmetic_overflow)]  

// TRY: run in debug mode, and then run in release mode

fn main() {

   let age:u8 = u8::MAX; //255

   // 0 to 255 only allowed for u8
   let x:u8 = u8::MAX +1 ;  //overflow value is 0
   // ALTERNATIVE MODIFICATIONS 
   // let x:u8 = age + 2 ;  // the compiler no longer statically detects the overflow
   // age.wrapping_add(1);  

   let y:u8 = u8::MAX + 2 ;  //overflow value is 1
   // ALTERNATIVE MODIFICATIONS 
   // let x:u8 = age + 2 ;  // the compiler no longer statically detects the overflow
   // age.wrapping_add(2);  

   println!("age is {} ",age);
   println!("x is {}",x);
   println!("y is {}",y);
}