fn foo(){
    fn main() {
    let z: i32 = 10;
    let v: &i32 = &z;
    let t = &v; // type: &&i32

    // Works: printing uses auto-deref (Display is on i32)
    println!("print: {}", t);

    /* ----------- artihmetic -------------- */
    // Doesn't compile: `t + 1` requires i32, auto-deref is NOT applied here
    // let sum = t + 1;

    // Works if you deref manually:
    let sum = **t + 1;
    println!("sum: {}", sum);

    /* ----------- comparison -------------- */

    // Doesn't compile: comparison expects i32, not &&i32
    // if t > 5 {}

    // Works with explicit deref
    if **t > 5 {
        println!("greater than 5");
    }
}

}


fn main() {
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
    /* All three give the same output because Rust bridges the gap with implicit dereferencing when needed. */

    // print the address instead of the value, notice the `:p` 
    println!("{:p}",  t);
    println!("{:p}", *t);
    println!("{}", **t);

    let x1 = *t;
}
