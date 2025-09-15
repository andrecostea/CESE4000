  fn example () -> i32 {               
    let a: i32 = 40;                   
    let b: i32 = 2;                    
    a + b                            
  }
  
  fn main () {                        
    let mut x: i32 = { example() + 6 };  // expression composition
    let y: &mut i32 = &mut x;            // mutable reference 
    *y += 5;                             // mutate the value stored in x
    println!("Hello, world: {}", y);     // print through the reference y
  }