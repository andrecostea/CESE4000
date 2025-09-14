
fn main() {
    let x: &Vec<i32>;
    //{        
      let y = vec![1, 2, 3];
      x = &y;  // Question related to mutability: why does this reference bind works althought x is declared as a RO variable
    //}        // if we uncomment the curly braces then y goes out of scope while x is still alive
    println!("{x:?}");
  }


