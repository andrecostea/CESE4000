
fn main() {

    /* --------- COPY --------- */
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    /* --------- MOVE --------- */
    // STRING
    let st = |x: &str| String::from(x); 
    let x_str = st("hello");
    let y_str = &x_str;

    assert_eq!(st("hello"), x_str);
    assert_eq!(st("hello"), *y_str);

    let x_str = st("hello");
    let y_str = Box::new(x_str);

    //assert_eq!(String::from("hello"), x_str);
    assert_eq!(st("hello"), *y_str);

    // VEC
    let x = vec![1,2,3];
    let y = &x;

    assert_eq!(vec![1,2,3], x);
    assert_eq!(vec![1,2,3], *y);

    let x = vec![1,2,3];
    let y = Box::new(x);

    //assert_eq!(vec![1,2,3], x);
    assert_eq!(vec![1,2,3], *y);
}
