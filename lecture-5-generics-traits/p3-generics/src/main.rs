/* I could instead have added a struct field for the REVERSED flag */
/* 
struct Packet<T, const N: usize>{
    /* other fields */
    data: [T; N],
    reverse: bool,
} 
*/
/* --------- Q: What's the difference between adding a struct field and adding a const generics? --------- */

#[derive(Debug)]
struct Packet<T, const N: usize, const REVERSED: bool> {
    /* other fields */
    data: [T; N],
}

struct Packet_mock<T, const N: usize> {
    /* other fields */
    data: [T; N],
}

// Adds a `new` and `reverse` behavior
impl <T, const N: usize, const REVERSED: bool> Packet<T, N, REVERSED> {
    fn new(mut data: [T;N]) -> Self {
        if REVERSED {                /* ---------  A: zero-cost abstraction---no runtime overhead --------- */
           data.reverse()
        };
        Packet { /* other fields */ data }
    }

    fn reverse(slice: &mut Self)  {
        if N > 0 { // avoid underflow if N=0
            let mut i = 0;
            let mut j = slice.data.len() - 1; 
            while i < j {
                slice.data.swap(i, j); 
                i += 1;
                j -= 1;
            }
        }
    }
}

type NumPacket  = Packet<i32,4, true>;
type CharPacket = Packet<char, 3, false>;


fn main(){
  let numbers = NumPacket::new([1, 2, 3, 4]);
  let letters = CharPacket::new(['a', 'b', 'c']);  

  println!("{:?}", numbers);
  println!("{:?}", letters);

}

