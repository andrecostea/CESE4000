
use std::ops::Rem;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::Rng;

/*
 credits: https://blog.jetbrains.com/rust/2024/03/12/rust-iterators-beyond-the-basics-part-ii-key-aspects/
 */

 // The pipeline encapsulates a heap-allocated Iterator trait object
 struct PipelineBuilder<'a, T> {
   iter: Box<dyn Iterator<Item = T> + 'a>
}

impl<'a, T: 'a> PipelineBuilder<'a, T> {
   fn new(iter: impl Iterator<Item = T> + 'a) -> Self {
       Self {
           iter: Box::new(iter)
       }
   }
}

impl<'a, T: 'a> PipelineBuilder<'a, T> {
   fn with_filter(self, cond: impl FnMut(&T) -> bool + 'a) -> Self {
       Self {
           iter: Box::new(self.iter.filter(cond))
       }
   }
   
   fn with_map<S>(self, f: impl FnMut(T) -> S + 'a) -> PipelineBuilder<'a, S> {
       PipelineBuilder {
           iter: Box::new(self.iter.map(f))
       }
   }

   fn for_each(self, f: impl FnMut(T) -> () + 'a) {
       self.iter.for_each(f)
   }
}

fn builder_files(){
  let filter = rand::thread_rng().gen_range(0..100).rem(2) == 0 ;
  let file = File::open("Cargo.toml").unwrap();
  let lines = BufReader::new(file).lines().map(Result::unwrap);
  let pipeline = PipelineBuilder::new(lines);

  if filter {
      pipeline.with_filter(|s| s.len() > 5 )
  } else {
     pipeline.with_map(|s| format!("{s}!"))
  }.with_map(|s| s.len()).for_each(|x| print!("{x} "));
}

fn builder_numbers(){
    let mut pipeline = PipelineBuilder::new((1..10).into_iter());

    let cnt = rand::thread_rng().gen_range(0..10) ;
    for i in 0..cnt {
        pipeline = pipeline.with_map(move |x| x + i)
    }
    pipeline
        .with_filter(|x| x.rem(2) == 0)
        .for_each(|x| print!("{x} "));
}

fn main() {
    let numbers = vec![1,2,3];
    let mut iter = numbers.iter();

/*     let filter =   rand::thread_rng().gen_range(0..100).rem(2) == 0 ;
    if filter {
        iter = iter.filter(|x| x.rem(2) == 0);
    } */
    iter.for_each(|x| print!("{x} "));

    println!();
    println!("Files pipeline:");
    builder_files();

    println!();
    println!("Numbers pipeline:");
    builder_numbers();
}



