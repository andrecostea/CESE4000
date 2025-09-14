fn main(){
 let x = vec![1, 2, 3];
 //let y = x; // MOVE, y becomes the owener of the [1,2,3] buffer
 println!("Here's your vector {x:?}"); // COMPILATION ERROR: x is not live after the move on line 3
}