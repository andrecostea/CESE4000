  fn print_option(x: Option<u32>) {
    match x {
      Some(value) => println!("the value inside is {value}"),
      None => println!("there was no value"),
    }
  }

  fn double_naive(input: Option<u32>) -> Option<u32> {
    if let Some(inner) = input {
      Some(inner * 2)
    } else {
      None
    }
  }

  fn double_closure(input: Option<u32>) -> Option<u32> {
    input.map(|inner| inner * 2)
  }

  fn double_macro(input: Option<u32>) -> Option<u32> {
    Some(input? * 2)
  }

  fn main(){
    print_option(double_naive(Some(2)));
  }