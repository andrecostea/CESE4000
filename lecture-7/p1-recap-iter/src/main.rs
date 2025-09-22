fn main() {
    
    /* --------------- dough to bread --------------- */
    let dough = vec![1,2,3];
    println!("dough {:?}", dough);
   // println!("dough first {:p}", &dough[0]);
    let b = |i| format!("b{}",i);
    let iter = dough.into_iter()
                    .map(|x| 
                        vec![b(x),b(x),]
                    )
                    .flatten();
    let mut bread = iter.collect::<Vec<String>>();
    println!("bread {:?}", bread);
   // println!("bread first {:p}", &bread[0]);
    //println!("dough {:?}", dough); // uncommenting this line would lead to a compilation error

    /* --------------- decorate bread --------------- */
    // bread is the owner of the new vector
    let iter = bread.iter_mut()
                    .map(|x| *x = x.to_uppercase()); // .map(|x| x.to_uppercase());
    iter.take(2).collect::<Vec<_>>();
    println!("BREAD: {:?}", bread);
    //println!("bread {:?}", iter.by_ref().take(2).collect::<Vec<_>>());
    //println!("bread {:?}", iter.take(2).collect::<Vec<_>>());

    /* --------------- analyse bread --------------- */
    // bread is still the owner of the bread
    let mut count = 0;
    let iter = bread.iter()
                    .inspect(|_| count += 1) 
                    .filter(|x| x.ends_with("2"));
    println!("Fancy breads: {}", iter.take(5).count()); // change this to 1
    println!("Number of iterations: {}", count);
}


/* 
Collection / Data
      │
      ▼
Iterator (state machine)
      │
      ▼
 ┌───────────────┐
 │   Adapters    │ ← Lazy, return new iterator, nothing happens yet
 │ ┌───────────┐ │
 │ │ map       │ │
 │ │ filter    │ │
 │ │ take      │ │
 │ │ enumerate │ │
 │ │ inspect   │ │
 │ └───────────┘ │
 └───────────────┘
      │
      ▼
 ┌───────────────┐
 │   Consumers   │ ← Eager, drives execution, produces final result
 │ ┌───────────┐ │
 │ │ collect   │ │
 │ │ sum       │ │
 │ │ count     │ │
 │ │ for_each  │ │
 │ │ find      │ │
 │ │ any/all   │ │
 │ └───────────┘ │
 └───────────────┘
      │
      ▼
 Final Value / Collection

*/

