use std::cmp::Ordering;
use std::cmp::{PartialEq, PartialOrd};
use std::collections::HashMap;
use std::rc::Rc;

use crate::lib::card::*;
use crate::lib::style::iface::*;
use crate::lib::style::CardStyle;

#[derive(Debug)]
pub struct Threes(pub Vec<[Card; 3]>);

impl Threes {
    pub fn to_style(cs: &Vec<Card>) -> Option<CardStyle> {
        let mut s = Threes(vec![]);
        let e = s.suit(&cs);
        if e == None {
            return Some(CardStyle::Threes(Rc::new(s)));
        }
        None
    }
}

impl StyleCmp for Threes {
    fn cmp(&self, cs: &Vec<Card>) -> Option<CardStyle> {
        let mut y = Threes(vec![]);
        let e = y.suit(&cs);
        if e.is_none() {
            if y > *(self) {
                return Some(CardStyle::Threes(Rc::new(y)));
            } else {
                return None;
            }
        }
        None
    }
}

impl Suit for Threes {
    type Error = &'static str;

    fn suit(&mut self, cs: &Vec<Card>) -> Option<Self::Error> {
        if cs.len() < 3 && cs.len() % 3 != 0 {
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

impl Layer for Threes {
    type Other = Threes;

    fn same_layer(&self, other: Self::Other) -> bool {
        self.0.len() == other.0.len()
    }
}

impl PartialEq for Threes {
    fn eq(&self, other: &Self) -> bool {
        self.0[0] == other.0[0]
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Threes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.gt(other) {
            Some(Ordering::Greater)
        } else if self.lt(other) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }

    fn gt(&self, other: &Self) -> bool {
        self.0[0] > other.0[0]
    }

    fn ge(&self, other: &Self) -> bool {
        self.0[0] >= other.0[0]
    }

    fn le(&self, other: &Self) -> bool {
        !self.gt(other)
    }

    fn lt(&self, other: &Self) -> bool {
        !self.ge(other)
    }
}
