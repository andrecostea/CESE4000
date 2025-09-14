/**
    T is a generic type, 
   `first` is a function with a generic type parameter (also called a polymorphic function)
*/
fn reverse<T>(slice: &mut [T]) {
    if slice.len() > 0 { // avoid underflow if N=0
        let mut i = 0;
        let mut j = slice.len() - 1; 
        while i < j {
            slice.swap(i, j); 
            i += 1;
            j -= 1;
        }
    }
}

// TODO: go to https://godbolt.org/z/Mz71shEr4
// to observe the monomorphization of `reverse` for the two required types

// The alternative would be to manually create two methods doing roughly the same thing:
fn reverse_i32(slice: &mut [i32]) {
    // ... <some lengthy pre-processing> ...
    if slice.len() > 0 {
        let mut i = 0;
        let mut j = slice.len()-1; 
        while i < j {
            slice.swap(i, j); // swap two elements
            i += 1;
            j -= 1;
        }
    }
}

fn reverse_char(slice: &mut [char]) {
    // ... <the same lengthy pre-processing> ...
    if slice.len() > 0 {
        let mut i = 0;
        let mut j = slice.len().saturating_sub(1); // avoid underflow if N=0
        while i < j {
            slice.swap(i, j); // swap two elements
            i += 1;
            j -= 1;
        }
    }
}

fn main(){
  let mut numbers = [1, 2, 3, 4];
  let mut letters = ['a', 'b', 'c'];  

  reverse(&mut numbers);           // results in: [4, 3, 2, 1];
  reverse(&mut letters);           // ['c', 'b', 'a']; 
  reverse(&mut [1.0, 1.5, 2.0]);   // [2.0, 1.5, 1.0]
  

  println!("{:?}", numbers);
  println!("{:?}", letters);
}

