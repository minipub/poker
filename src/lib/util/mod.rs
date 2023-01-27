use std::collections::HashMap;

use crate::lib::card::Card;

pub fn card_buckets(cs: &Vec<Card>) -> HashMap<u8, Box<Vec<Card>>> {
    let mut buckets: HashMap<u8, Box<Vec<Card>>> = HashMap::new();

    for x in cs.iter() {
        let k = x.unwrap_point();
        if !buckets.contains_key(&k) {
            buckets.insert(k, Box::new(vec![]));
        }
        buckets.get_mut(&k).unwrap().push(*x);
    }

    println!("buckets: {:?}", buckets);

    buckets
}
