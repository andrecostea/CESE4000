use std::collections::{LinkedList,HashMap,HashSet,BTreeMap,BTreeSet};

fn arr_classic() {
    let mut data = [1,2,3];  // [1, 4 ,9]

    print!("{} ", data[0] * data[0]);
    print!("{} ", data[1] * data[1]);
    print!("{} ", data[2] * data[2]);
    println!();
    
    // ----------- squares -----------
    println!("squares: ");
    let mut idx = 0;
    while idx < 3 {
        let temp = data[idx] * data[idx]; 
        print!("{} ", temp);
        idx += 1;
    }
    println!();

    // ------------- sum -------------
    println!("sum in an array: ");
    let mut idx = 0;
    let mut sum = 0;
    while idx < 3 {
        sum += data[idx]; 
        idx += 1;
    }
    println!("{}",sum); println!();

    // ------- in-place update -------
    println!("in-place update: ");
    let mut idx = 0;
    while idx < 3 {
        data[idx] = data[idx] * 10; 
        idx += 1;
    } 
    println!("{:?}",data); println!();

    // ------------ clone ------------
    println!("clone: ");
    let mut idx = 0;
    let mut clone = [0; 3];
    while idx < 3 {
        clone[idx] = data[idx]; 
        idx += 1;
    }
    println!("{:?}", clone); println!();

}

fn arr_iter() {
    let mut data = [1,2,3];  // [1, 4 ,9]

    // ----------- squares -----------
    println!("squares: ");
    data.iter()
        .map(|&x| x * x )
        .for_each(|temp| print!("{} ", temp));
    println!();

    // ------------- sum -------------
    println!("sum in an array: ");
    let mut sum = 0;
    data.iter()
        .for_each(|item| sum += item);
    println!("{}",sum); println!();

    println!("sum in an array: (alternative) ");
    let sum = data
        .iter()
        .fold(0,|acc, item| acc + item);
    println!("{}",sum); println!();

    // ------- in-place update -------
    println!("in-place update: ");
    data.iter_mut()
        .for_each(|item| *item = *item * 10);
    println!("{:?}",data); println!();

    // ------------ clone ------------
    println!("clone: ");
    let mut clone = [0; 3];
    data.iter()
        .enumerate()
        .for_each(|(i, &x)| clone[i] = x);
    println!("{:?}", clone); println!();
  }

  fn many_iter(){
    // ------------ array ------------
    let data = [1,2,3];  
    data.iter().for_each(|temp| print!("arr {} ", temp));
    println!();println!();
    
    // ------------ vec ------------
    let data = vec![1,2,3];  
    data.iter().for_each(|temp| print!("vec {} ", temp));
    println!();println!();

    // ------------ linkedlist ------------
    let list = LinkedList::from([1,2,3]);
    list.iter().for_each(|x| println!("linekdlist {}", x));
    println!();

    // ------------ hashmap ------------
    let hmap = HashMap::from([(1,"a"), (2, "b"), (3, "c"),]);
    hmap.iter().for_each(|(k, v)| println!("hashmap {} -> {}", k, v));
    println!();

    // ------------ hashset ------------ 
    let set: HashSet<_> = HashSet::from([1,2,3]);
    set.iter().for_each(|x| println!("hashset: {}", x));
    println!();

    // ------------ btreeset ------------ 
    let bset: BTreeSet<_> = BTreeSet::from([3, 2, 1]);
    bset.iter().for_each(|x| println!("btreeset: {}", x));
    println!();

    // ------------ btreemap ------------ 
    let bmap: BTreeMap<_, _> = BTreeMap::from([("x", 1), ("y", 1)]);
    bmap.iter().for_each(|(k, v)| println!("btreemap: {} -> {}", k, v));
    println!();

  }

fn main() {

    arr_classic();

    arr_iter();

    many_iter();
}
