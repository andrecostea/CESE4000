/* --------- Q:  What makes this program safe? --------- */
fn main(){
    let mut x = vec![10, 11];

    let v = &mut x;   // lifetime of the OUTER reference STARTS here -- mutable borrow

    let i = &mut (*v)[0];  // lifetime of the INNER reference STARTS here -------- A: reborrow --------

    println!("x[0] = {}", *i);       // lifetime of the INNER reference ENDS here

    Vec::push(v, 12);    // lifetime of the OUTER reference ENDS here

    // test out reborrow and reference cast

    foo()
}


/* ------------ reference cast --------------- */

fn foo(){
    let x: i32 = 0;
    let y: &i32      = &&&&&&x; /* -------- Q: What's happening in here? -------- */
    let z: &&&&&&i32 = &&&&&&x; 
    println!("{}", y);
    println!("{}", z);
    
    let z: i32 = 10;
    let v: &i32 = &z;
    let t = &v;
    
    println!("{}", t);
    println!("{}", *t);
    println!("{}", **t);     /* -------- Q: How come they all print the same? -------- */

    // print the address instead of the value, notice the `:p` 
    println!("{:p}",  t);
    println!("{:p}", *t);
    println!("{}", **t);

    let x1 = *t;

}
