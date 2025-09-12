fn main() {
    let v = vec![10, 20, 30];
    let def = 0;
    let ref_d = &def;

    let r: &i32 =
       if v.len()>0 { &v[0] }
       else  { ref_d } ;

    println!("{}", r);
}