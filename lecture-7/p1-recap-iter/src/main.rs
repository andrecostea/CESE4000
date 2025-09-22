fn main() {
    
    let dough = vec![1, 2, 3];

    /* --------------- dough to bread --------------- */
    let iter = dough.into_iter()
                    .map(|x| vec![format!("b{x}"), format!("p{x}")])
                    .flatten();
    let mut bread = iter.collect::<Vec<String>>();
    println!("Bread: {:?}", bread);

    /* --------------- decorate bread --------------- */
    let mut iter = bread.iter_mut()
                    .map(|x| *x = x.to_uppercase() );
    iter.by_ref().take(2).collect::<Vec<_>>();
    iter.take(2).collect::<Vec<_>>();
    println!("Bread: {:?}", bread);

    /* --------------- analyse bread --------------- */
    let mut count = 0;
    let iter = bread.iter()
                    .filter(|x| x.ends_with("2"))
                    .inspect(|_| count += 1);
    iter.take(4).collect::<Vec<_>>();

    println!("count {count}");
    
}