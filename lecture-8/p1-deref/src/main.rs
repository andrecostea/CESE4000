
fn print(x: &str) {
   println!("Software {x}!");
}

fn main(){
let m = Box::new(String::from("Fundamentals"));

print(&m);
//    ^^ &Box<String>  ---(deref)--->  &String  ---(deref)--->  &str
print("!!");

}