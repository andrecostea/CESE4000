// a bespoke collection
struct MyCollection<T> {
    data: Vec<T>,
}

// a borrowing iterator
struct MyCollectionIter<'a, T> {
    collection: &'a MyCollection<T>,
    index: usize,
}

impl<'a, T> Iterator for MyCollectionIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.collection.data.len() {
            None
        } else {
            let item = &self.collection.data[self.index];
            self.index += 1;
            Some(item)
        }
    }
}

// implement IntoIterator for references
impl<'a, T> IntoIterator for &'a MyCollection<T> {
    type Item = &'a T;
    type IntoIter = MyCollectionIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        MyCollectionIter {
            collection: self,
            index: 0,
        }
    }
}


fn main() {
    let my_col = MyCollection { data: vec![10, 20, 30] };

    for x in &my_col {
        println!("{x}");
    }

    // Collection is still usable
    println!("Length: {}", my_col.data.len());
}
