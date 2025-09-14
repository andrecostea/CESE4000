use std::fmt::Display;

//#[derive(Debug)]
/** 
   A struct generic over a type T and a constant N 
*/
struct Packet<T: ToString, const N: usize>{
    /* other fields */
    data: [T; N],
}

/** 
   An implementation of the `new` and `reverse` and `print` behaviours that would 
   be shared across all instances of Packet 
*/
impl <T: ToString, const N: usize> Packet<T,N> {
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

    fn print(&self) {
        print!("New Packet: ");
        for item in &self.data {
            print!("{} ", item.to_string());
        }
        println!();
    }
}

/** 
   Creates type aliases for two concrete types / specialized structs 
*/
type NumPacket  = Packet<i32,4>;
type CharPacket = Packet<char, 3>;

struct ID(char);
type MyCharPacket = Packet<ID, 3>;

impl ToString for ID{
    fn to_string(&self) -> String {
        format!("{}", self.0)  
    }
}

fn main(){
  let mut numbers  = NumPacket::new([1, 2, 3, 4]);
  let mut letters = CharPacket::new(['a', 'b', 'c']);  
  let mut caps= MyCharPacket::new([ID('a'), ID('b'), ID('c')]);

  NumPacket::reverse(&mut numbers);
  CharPacket::reverse(&mut letters);

  //println!("{:?}", numbers);
  //println!("{:?}", letters);

  numbers.print();
  letters.print();
  caps.print();
}

