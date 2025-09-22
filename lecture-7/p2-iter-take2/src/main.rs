fn main() {
    let dough = vec![1,2,3];

    /* **** DOUGH TO BREAD ***** */
    let iter = dough.into_iter()
                    .map(|x| vec![format!("b{x}"),format!("p{x}"),])
                    .flatten();
    let mut bread = iter.collect::<Vec<String>>();
    println!("bread: {:?}", bread);
    

    /* ***** DECORATE BREAD ***** */
    let mut iter = bread.iter_mut()
                    .map(|x| *x = x.to_uppercase());
    // iter.by_ref().take(2).collect::<Vec<_>>();  // Use by_ref to use the iterator again
    iter.take(2).collect::<Vec<_>>(); /* -------- Q: What happens if I try to use iter again after collecting? -------- */
    //resume iteration where it left off
    println!("decorated bread: {:?}", bread);

    /* ******* INSPECT BREAD ***** */

    let mut count = 0;

/* first show this, as expected -- notice the inspect and filter are swapped */
/*     let iter = bread.iter()
                    .filter(|x| x.ends_with("2"))
                    .inspect(|_| count += 1);
    iter.take(1).collect::<Vec<_>>();
    iter.take(2).collect::<Vec<_>>();
    println!("fancy breads: {count}"); */

    let iter = bread.iter()
                    .inspect(|_| count += 1)
                    .filter(|x| x.ends_with("2"));
    //iter.take(5).collect::<Vec<_>>();  // why 6?
    iter.take(1).collect::<Vec<_>>();    // why 3?
    println!("fancy breads: {count}")
}
