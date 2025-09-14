#[derive(Debug)]
/** 
   A struct generic over a type T and a constant N 
*/
struct Packet<T, const N: usize>{
    /* other fields */
    data: [T; N],
}

/** 
   An implementation of the `new` and `reverse` behaviours that would 
   be shared across all instances of Packet 
*/
impl <T, const N: usize> Packet<T,N> {
    fn new(data: [T;N]) -> Self {
       Packet {/* other fields */ data}
    }

    fn reverse(slice: &mut Self) {
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

/** 
   Creates type aliases for two concrete types / specialized structs 
*/
type NumPacket  = Packet<i32,4>;
type CharPacket = Packet<char, 3>;


fn main(){
  let mut numbers  = NumPacket::new([1, 2, 3, 4]);
  let mut letters = CharPacket::new(['a', 'b', 'c']);  

  NumPacket::reverse(&mut numbers);
  CharPacket::reverse(&mut letters);

  println!("{:?}", numbers);
  println!("{:?}", letters);
}

