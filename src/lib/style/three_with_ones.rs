use std::cmp::Ordering;
use std::cmp::{PartialEq, PartialOrd};
use std::collections::HashMap;
use std::rc::Rc;

use crate::lib::card::*;
use crate::lib::style::iface::*;
use crate::lib::style::CardStyle;

#[derive(Debug)]
pub struct ThreeWithOnes(pub Vec<([Card; 3], [Card; 1])>);

impl ThreeWithOnes {
    pub fn to_style(cs: &Vec<Card>) -> Option<CardStyle> {
        let mut s = ThreeWithOnes(vec![]);
        let e = s.suit(&cs);
        if e == None {
            return Some(CardStyle::ThreeWithOnes(Rc::new(s)));
        }
        None
    }
}

impl Suit for ThreeWithOnes {
    type Error = &'static str;

    fn suit(&mut self, cs: &Vec<Card>) -> Option<Self::Error> {
        if cs.len() < 4 && cs.len() % 4 != 0 {
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

        if buckets.len() % 2 == 1 {
            return Some("ThreeWithOnes number must be plural.");
        }

        let mut threes: Vec<[Card; 3]> = Vec::new();
        let mut ones: Vec<[Card; 1]> = Vec::new();

        for (k, v) in buckets {
            if v.len() == 3 {
                threes.push([v[0], v[1], v[2]]);
            } else if v.len() == 1 {
                ones.push([v[0]]);
            } else {
                return Some("ThreeWithOnes must be consist of 3s and 1.");
            }
        }

        if threes.len() != ones.len() {
            return Some("ThreeWithOnes must be consist of pairs of 3s and 1.");
        }

        // println!("a threes: {:?}", threes);
        threes.sort_by(|x, y| x[0].partial_cmp(&y[0]).unwrap());
        // println!("b threes: {:?}", threes);

        if threes.len() > 1 {
            let mut i = 0;
            while i < threes.len() - 1
                && threes[i][0].unwrap_point() + 1 == threes[i + 1][0].unwrap_point()
            {
                i += 1;
            }

            if i + 1 != threes.len() {
                return Some("ThreeWithOnes must be consist of continuous threes.");
            }
        }

        let mut vs: Vec<([Card; 3], [Card; 1])> = vec![];

        while threes.len() > 0 {
            vs.push((threes.remove(0), ones.remove(0)));
        }

        self.0 = vs;

        None
    }
}

impl Layer for ThreeWithOnes {
    type Other = ThreeWithOnes;

    fn same_layer(&self, other: Self::Other) -> bool {
        self.0.len() == other.0.len()
    }
}

impl PartialEq for ThreeWithOnes {
    fn eq(&self, other: &Self) -> bool {
        self.0[0].0[0] == other.0[0].0[0]
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for ThreeWithOnes {
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
        self.0[0].0[0] > other.0[0].0[0]
    }

    fn ge(&self, other: &Self) -> bool {
        self.0[0].0[0] >= other.0[0].0[0]
    }

    fn le(&self, other: &Self) -> bool {
        !self.gt(other)
    }

    fn lt(&self, other: &Self) -> bool {
        !self.ge(other)
    }
}
