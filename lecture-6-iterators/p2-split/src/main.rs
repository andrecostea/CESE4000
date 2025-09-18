fn closure_example() {
    let sentence = "The wind is gentle, until it isn't!";

    // Split by different characters
    let words= sentence.split(|c| c == ',' || c == '!' || c == ' ').collect::<Vec<_>>();

    println!("{:?}", words); // ["The", "wind", "is", "gentle", "until", "it", "isn't"]
    
    // Iterate without collecting
    for word in sentence.split(|c| c == ',' || c == '!' || c == ' ') {
        println!("{}", word);
    }
}

fn main() {
    let sentence = "The wind is gentle";
    
    let characters: Vec<char> = sentence.chars().collect();
    println!("{:?}", characters);
    // ['T', 'h', 'e', ' ', 'w', 'i', 'n', 'd', ' ', 'i', 's', ' ', 'g', 'e', 'n', 't', 'l', 'e']

    // Split by space ' '
    let words: Vec<&str> = sentence.split(' ').collect();
    println!("{:?}", words); // ["The", "wind", "is", "gentle"]

    let &[w1, w2, w3, w4] = sentence
                                                           .split(' ')
                                                            .collect::<Vec<_>>()
                                                            .as_slice() else {
        todo!();
    }; 
    
    // Iterate without collecting
    for word in sentence.split(' ') {
        println!("{}", word);
    }

}
