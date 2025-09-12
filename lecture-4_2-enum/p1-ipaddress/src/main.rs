#[derive(Debug)]
enum IpAddress { 
  Ipv4(u8, u8, u8, u8),                           
  Ipv6(u16, u16, u16, u16, u16, u16, u16, u16),   
}
enum HomeAddress {
    Ipv4Home,
    Ipv6Home,
    NotHome,
}

fn accept_ipv4(ip: IpAddress) {
  if let IpAddress::Ipv4(a, b, _, _) = ip {
    println!("Accepted, first octet is {} and second is {}", a, b);
  }
}

fn accept_home(ip: IpAddress) -> HomeAddress {
  match ip {
    IpAddress::Ipv4(127, 0, 0, 1) => HomeAddress::Ipv4Home,
    IpAddress::Ipv6(0, 0, 0, 0, 0, 0, 0, 1) => HomeAddress::Ipv6Home,
    _ => HomeAddress::NotHome,
  }
}

fn get_first_byte(ip: IpAddress) {

  let first_byte = match ip {
    IpAddress::Ipv4(a, _, _, _) => a,
    IpAddress::Ipv6(a, _, _, _, _, _, _, _) => (a / 256) as u8,
  };

  println!("The first byte is: {}", first_byte);
}

fn cu(x: u8) -> u32 { x as u32 }

impl IpAddress {
 fn as_u32(self) -> Option<u32> {
  match self {
    IpAddress::Ipv4(a,b,c,d) => Some(
                cu(a) << 24 |
                cu(b) << 16 |
                cu(c) << 8  |
                cu(d) 
            ),
    _ => None,
  }
 }
}

impl From<[u8; 4]> for IpAddress {
    fn from(octets: [u8; 4]) -> Self {
        IpAddress::Ipv4(octets[0], octets[1], octets[2], octets[3])
    }
}

fn main() {
  let ipv4_home = IpAddress::Ipv4(127, 0, 0, 1);
  let ipv6_home = IpAddress::Ipv6(0, 0, 0, 0, 0, 0, 0, 1);

   let ipv4 = ipv4_home.as_u32();
   println!("{:?}", ipv4); 
   assert_eq!(ipv4, Some(0x7f000001));

   let addr = IpAddress::from([127, 0, 0, 1]);
   println!("{:?}", addr); 
   
}


