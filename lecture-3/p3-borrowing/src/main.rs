
fn main() {
    let x: &Vec<i32>;
    //{        // if we uncomment the curly braces then y goes out of scope
      let y = vec![1, 2, 3];
      x = &y;  // why does this reference bind works althought x is declared as a RO variable
    //}
    println!("{x:?}");
  }


