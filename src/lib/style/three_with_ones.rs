use std::collections::HashMap;

use crate::lib::card::*;
use crate::lib::style::iface::*;

#[derive(Debug)]
pub struct ThreeWithOnes(pub Box<[([Card; 3], [Card; 1])]>);

impl Suit for ThreeWithOnes {
    type Error = &'static str;

    fn suit(&mut self, cs: Box<Vec<Card>>) -> Option<Self::Error> {
        if cs.len() % 4 != 0 {
            return Some("ThreeWithOnes number must be 4s.");
        }

        let mut buckets: HashMap<u8, Box<Vec<Card>>> = HashMap::new();

        for x in cs.iter() {
            let k = x.unwrap_point();
            if !buckets.contains_key(&k) {
                buckets.insert(k, Box::new(vec![]));
            }
            buckets.get_mut(&k).unwrap().push(*x);
        }

        println!("buckets: {:?}", buckets);

        None
    }
}
