fn main(){
    let mut x = vec![10, 11];

    let v = &mut x;

    let i = &mut (*v)[0];

    println!("x[0] = {}", *i);

    Vec::push(v, 12);
}