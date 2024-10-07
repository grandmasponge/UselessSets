use std::collections::HashSet;
use crate::elements::{Elements, SetDataTypes};
use crate::sets::SetOperations;

mod sets;
mod elements;
mod error;

fn main() {
    let se1 = HashSet::from([Elements::from(1), Elements::from("hello world".to_string())]);
    let mut s1 = sets::Set::SetfromHashSet(se1);
    let se2 = HashSet::from([Elements::from(1), Elements::from("hello terry".to_string()), Elements::from('a')]);
    let mut s2 = sets::Set::SetfromHashSet(se2);

    //you can add elements of any data type that is so far implimented

    s1.add(&[Elements::from('a'), Elements::from("eh".to_string())]).unwrap();
    //you can remove elements as well

    s1.remove(&[Elements::from(1)]).unwrap();

    let union_s1_s2  = s1.union(&s2).unwrap();
    let difference = s1.difference(&s2).unwrap();
    let intersection = s1.intersection(&s2).unwrap();

    println!("union: {:?}", union_s1_s2.elements);
    println!("difference: {:?}", difference.elements);
    println!("intersection: {:?}", intersection.elements);

}
