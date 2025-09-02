  fn example () -> i32 {               
    let a: i32 = 40;                   
    let b: i32 = 2;                    
    a + b                            
  }
  
  fn main () {                        
    let mut x: i32 = { example() + 6 };
    let y: &mut i32 = &mut x;          
    *y += 5;                             
    println!("Hello, world: {}", y);   
  }