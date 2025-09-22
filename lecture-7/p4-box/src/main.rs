
// Each node contains data and a pointer to the next node
struct Node {
    data: i32,
    next: Option<Box<Node>>, // None for the last node
}

// Optional: helper type for easier readability
type LinkedList = Option<Box<Node>>;


fn main() {
  let x = 2;
  let b = Box::new(5);
  println!("b = {}", b);

let list: LinkedList = Some(Box::new(Node {
    data: 1,
    next: Some(Box::new(Node {
        data: 2,
        next: Some(Box::new(Node {
            data: 3,
            next: None,
        })),
    })),
}));

// Traverse and print
let mut current = &list;
while let Some(node) = current {
    println!("{}", node.data);
    current = &node.next;
}

}

