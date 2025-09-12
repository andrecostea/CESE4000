 fn main(){
  let mut x = vec![1, 2, 3];

  let a = &mut x;
  let b = a; // a moved into b, not copied

  // so a is not valid anymore here
  //a.push(4);    // Err: borrow of moved value: `a`
  b.push(5);
 }