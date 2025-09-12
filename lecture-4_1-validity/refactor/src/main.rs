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
    
    let r: &i32 =              // get_first(&v, ref_d);
      if v.len()>0 { &v[0] }
       else  { ref_d } ;

    println!("{}", r);
}