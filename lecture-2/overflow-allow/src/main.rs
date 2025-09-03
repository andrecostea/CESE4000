//#[allow(arithmetic_overflow)]
fn main(){

    let age:u8 = u8::MAX; //255

    let x:u8 = age.wrapping_add(1);   //overflow value is 0
    let y:u8 = age.wrapping_add(2);   //overflow value is 1
    
    println!("age is {} ",age);
    println!("x is {}",x);
    println!("y is {}",y);
}