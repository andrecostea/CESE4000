fn main() {
    // type is [i32; 3], meaning "an array of 3 i32s" allocated on the stack, fixed size
    let arr: [i32; 3] = [1, 2, 3]; 

    let first = arr[0];
    let second = arr[1];

    println!("Array: {:?}", arr); // [1, 2, 3]

    // type is Vec<i32>, meaning "a buffer of 3 i32s" allocated on the heap, can change size
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(4);   // Vec can grow
    v.pop();     // Vec can shrink
    println!("Vector: {:?}", v); // [1, 2, 3]

}