#![allow(dead_code)]
use std::collections::btree_map::BTreeMap;

fn main() {
    let mut count = BTreeMap::new();
    let message = "she sells sea shells by the sea shore";

    for c in message.chars() {
        *count.entry(c).or_insert(0) += 1;
    }

    assert_eq!(count.get(&'s'), Some(&8));

    println!("number of occurrences of each character");
    for (char, count) in &count {
        println!("{char}: {count}");
    }
}











































