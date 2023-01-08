use std::collections::HashMap;

use crate::lib::card::*;
use crate::lib::style::iface::Suit;

#[derive(Debug)]
struct Threes(pub Vec<[Card; 3]>);

impl Suit for Threes {
    type Error = &'static str;

    fn suit(&mut self, cs: Box<Vec<Card>>) -> Option<Self::Error> {
        if cs.len() > 0 && cs.len() % 3 != 0 {
            return Some("Threes number must be 3s.");
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

        let mut threes: Vec<[Card; 3]> = Vec::new();

        for (k, v) in buckets {
            if v.len() == 3 {
                threes.push([v[0], v[1], v[2]]);
            } else {
                return Some("Threes must be consist of 3s.");
            }
        }

        // println!("a threes: {:?}", threes);
        threes.sort_by(|x, y| x[0].partial_cmp(&y[0]).unwrap());
        // println!("b threes: {:?}", threes);

        self.0 = threes;

        None
    }
}
