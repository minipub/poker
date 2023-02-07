use std::cmp::Ordering;
use std::cmp::{PartialEq, PartialOrd};
use std::rc::Rc;

use crate::lib::card::*;
use crate::lib::style::iface::*;
use crate::lib::style::CardStyle;

#[derive(Debug)]
pub struct Chain(pub Vec<Card>);

impl Chain {
    pub fn to_style(cs: &Vec<Card>) -> Option<CardStyle> {
        let mut s = Chain(vec![]);
        let e = s.suit(&cs);
        if e == None {
            return Some(CardStyle::Chain(Rc::new(s)));
        }
        None
    }
}

impl StyleCmp for Chain {
    fn cmp(&self, cs: &Vec<Card>) -> Option<CardStyle> {
        let mut y = Chain(vec![]);
        let e = y.suit(&cs);
        if e.is_none() {
            if y > *self {
                return Some(CardStyle::Chain(Rc::new(y)));
            } else {
                return None;
            }
        }
        None
    }
}

impl Suit for Chain {
    type Error = &'static str;

    fn suit(&mut self, cs: &Vec<Card>) -> Option<Self::Error> {
        if cs.len() < 5 {
            return Some("not reach 5 elements.");
        }

        let mut v = vec![Card::default(); cs.len()];
        v.clone_from_slice(&cs);

        // println!("before sort, v: {:?}", v);
        v.sort_by(|x, y| x.partial_cmp(y).unwrap());
        // println!("after sort, v: {:?}", v);

        let mut m = v[0].unwrap_point();
        for x in &v {
            let xp = x.unwrap_point();
            if m != xp {
                return Some("not continous.");
            }
            m = xp + 1;
        }

        self.0 = v;
        None
    }
}

impl Layer for Chain {
    type Other = Chain;

    fn same_layer(&self, other: Self::Other) -> bool {
        self.0.len() == other.0.len()
    }
}

impl PartialEq for Chain {
    fn eq(&self, other: &Self) -> bool {
        self.0[0] == other.0[0]
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Chain {
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
