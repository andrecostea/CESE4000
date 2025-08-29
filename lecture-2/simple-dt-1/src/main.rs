  // compiler, when I call `foo` somewhere in my code
  fn foo() {
    // please make some room for me to use 4 bytes for something
    // I'll use the name `a` when I want to use it
    let a: u32 = 3;

    // and 16 more here, I'll call it b
    let b: u128 = 100_000;
  }

fn main() {
        foo();
}