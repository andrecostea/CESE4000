#[allow(arithmetic_overflow)]
fn main(){
    use std::num::Wrapping;

    let age:u8 = u8::MAX; //255

    let x:Wrapping<u8> = Wrapping(u8::MAX + 1);   //overflow value is 0
    let y:Wrapping<u8> = Wrapping(u8::MAX + 2);   //overflow value is 1
    
    println!("age is {} ",age);
    println!("x is {}",x);
    println!("y is {}",y);
}