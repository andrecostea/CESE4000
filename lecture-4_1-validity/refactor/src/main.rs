/*
// NOTE: the lifetimes could all be different as long as we add the constraint that v and x must outlive the return reference
fn get_first<'a,'b, 'c>(v: &'a Vec<i32>, x:  &'b i32) -> &'c i32  
  where
      'a: 'c,
      'b: 'c, 
{
    if v.len()>0 { &v[0] }
    else { x }
}
*/

/* 
fn get_first<'a>(v: &'a Vec<i32>, x:  &'a i32) -> &'a i32 {
    if v.len()>0 { &v[0] }
    else { x }
}
*/

/* 
fn get_firstt(v: &Vec<i32>, x:  &i32) -> &i32 {
    if v.len()>0 { &v[0] }
    else { x }
}
*/    

fn main() {
    let v: Vec<i32> = vec![10, 20, 30];
    let def: i32 = 0;
    
    let ref_d: &i32 = &def;
    
    let r: &i32 =             // get_first(&v, ref_d);
      if v.len()>5 { &v[0] }
      else  { &ref_d } ;      // NOTE: Writing &ref_d gives you a &&i32, but Rust coerces it down to &i32 automatically, because both arms of the if must have the same type, which is &i32.

    println!("{}", r);
}