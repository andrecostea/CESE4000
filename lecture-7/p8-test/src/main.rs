
/* **** DOUGH TO BREAD ***** */
pub fn dough_to_bread(dough: Vec<i32>) -> Vec<String>{
    let iter = dough.into_iter()
                    .map(|x| vec![format!("b{x}"),format!("p{x}"),])
                    .flatten();
    let bread = iter.collect::<Vec<String>>();
    println!("bread: {:?}", bread);
    bread
}

/* ***** DECORATE BREAD ***** */
pub fn decorate_bread(bread: &mut Vec<String>){
    let mut iter = bread.iter_mut()
                    .map(|x| *x = x.to_uppercase());
    iter.by_ref().take(2).collect::<Vec<_>>();
    iter.take(2).collect::<Vec<_>>();
    println!("decorated bread: {:?}", bread)
}

/* ******* INSPECT BREAD ***** */
pub fn inspect_bread(bread: &Vec<String>, n: usize) -> i32{
    let mut count = 0;
    let iter = bread.iter()
                    .filter(|x| x.ends_with("2"))
                    .inspect(|_| count += 1);
    iter.take(n).collect::<Vec<_>>();    
    count
}

fn main() {
    let dough = vec![1,2,3];
    let mut bread = dough_to_bread(dough);
    decorate_bread(&mut bread);
    let count = inspect_bread(&bread,2);

    println!("fancy breads: {count}");
}


#[cfg(test)]
mod test{
    use proptest::prelude::*;
    use crate::{decorate_bread, dough_to_bread, inspect_bread};

    // UNIT TEST
    #[test]
    fn d2b(){
        assert!(dough_to_bread(vec![1,2,3]).len()>0);
        assert!(dough_to_bread(vec![1,2,3]).len() == 6);
        assert_eq!(dough_to_bread(vec![1]), vec!["b1", "p1"]);
    }

    // INTEGRATION TEST
    #[test]
    fn decor(){
        let dough = vec![1];
        let mut bread = dough_to_bread(dough);

        decorate_bread(&mut bread);
        assert!(bread[0].starts_with("B"));

        decorate_bread(&mut bread);
        assert_eq!(bread, vec!["B1", "P1"]);
    }

    // INTEGRATION TEST
    #[test]
    fn analyse(){
        let dough = vec![1];
        let mut bread = dough_to_bread(dough);

        decorate_bread(&mut bread);
        assert!(bread[0].starts_with("B"));

        let count = inspect_bread(&bread, 2);
        assert_eq!(count, 0);
    }

    proptest! {
        // property: every input number `x` produces *two strings*: "bx" and "px"
        #[test]
        fn dough_to_bread_has_twice_length(xs: Vec<i32>) {
            let bread = dough_to_bread(xs.clone());

            // The output must be twice the input length
            prop_assert_eq!(bread.len(), xs.len() * 2);
            println!("{:?} ", bread);

            // Check the format of produced elements
            for (i, x) in xs.into_iter().enumerate() {
                prop_assert_eq!(&bread[2*i], &format!("b{x}"));
                prop_assert_eq!(&bread[2*i+1], &format!("p{x}"));
            }
        }
    }

    // ---- decorate_bread ----
    proptest! {
        // property: decorating bread uppercases some elements
        #[test]
        fn decorate_bread_makes_uppercase( //mut xs: Vec<i32>) 
             mut bread in proptest::collection::vec("[a-z]{1,3}", 4) // fixed-size 4 Vec<String>
        ) 
        {
            decorate_bread(&mut bread);

            // At least the first two should be uppercase
            for item in bread.iter().take(2) {
                prop_assert!(item.chars().all(|c| c.is_uppercase()));
            }
        }
    }

    // ---- inspect_bread ----
     proptest! {
        // property: inspect_bread counts total items *seen* by filter, not just collected
        #[test]
        fn inspect_bread_counts_items( //xs: Vec<i32>) {
            xs in proptest::collection::vec(0i32..100, 2..=10)) // vector of ints, size 2–10
        {
            let bread = dough_to_bread(xs.clone());
            let count = inspect_bread(&bread,2);

            // count must be equal to bread.len() if since inspect runs before filter
            // prop_assert!(count == bread.len() as i32);
            
            prop_assert!(count <= bread.len() as i32);
        }
    } 
}

