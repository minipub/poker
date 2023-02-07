use std::cmp::Ordering;
use std::cmp::{PartialEq, PartialOrd};
use std::rc::Rc;

use crate::lib::card::*;
use crate::lib::style::iface::*;
use crate::lib::style::CardStyle;

#[derive(Debug)]
pub struct Pairs(pub Vec<[Card; 2]>);

impl Pairs {
    pub fn to_style(cs: &Vec<Card>) -> Option<CardStyle> {
        let mut s = Pairs(vec![]);
        let e = s.suit(&cs);
        if e == None {
            return Some(CardStyle::Pairs(Rc::new(s)));
        }
        None
    }
}

impl StyleCmp for Pairs {
    fn cmp(&self, cs: &Vec<Card>) -> Option<CardStyle> {
        let mut y = Pairs(vec![]);
        let e = y.suit(&cs);
        if e.is_none() {
            if y > *(self) {
                return Some(CardStyle::Pairs(Rc::new(y)));
            } else {
                return None;
            }
        }
        None
    }
}

impl Suit for Pairs {
    type Error = &'static str;

    fn suit(&mut self, cs: &Vec<Card>) -> Option<Self::Error> {
        if cs.len() < 2 {
            return Some("chain pairs number must be ge 2.");
        }
        if cs.len() == 4 {
            return Some("chain pairs number can't be 4.");
        }
        if cs.len() % 2 == 1 {
            return Some("chain pairs number must be plural.");
        }

        let mut v = vec![Card::default(); cs.len()];
        v.clone_from_slice(&cs);

        // println!("before sort, v: {:?}", v);
        v.sort_by(|x, y| x.partial_cmp(y).unwrap());
        // println!("after sort, v: {:?}", v);

        let mut v2 = vec![[Card::default(); 2]; cs.len() / 2];
        let mut m1 = v[0].unwrap_point();
        let mut m2 = v[1].unwrap_point();
        let mut i = 0;

        while i < cs.len() {
            let xp = v[i].unwrap_point();
            let yp = v[i + 1].unwrap_point();

            if xp != yp || m1 != m2 {
                return Some("not a pair.");
            }

            if m1 != xp {
                return Some("not continous.");
            }

            v2[i / 2] = [v[i], v[i + 1]];

            m1 = xp + 1;
            m2 = m1;
            i += 2;
        }

        self.0 = v2;

        None
    }
}

impl Layer for Pairs {
    type Other = Pairs;

    fn same_layer(&self, other: Self::Other) -> bool {
        self.0.len() == other.0.len()
    }
}

impl PartialEq for Pairs {
    fn eq(&self, other: &Self) -> bool {
        self.0[0][0] == other.0[0][0]
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Pairs {
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
        self.0[0][0] > other.0[0][0]
    }

    fn ge(&self, other: &Self) -> bool {
        self.0[0][0] >= other.0[0][0]
    }

    fn le(&self, other: &Self) -> bool {
        !self.gt(other)
    }

    fn lt(&self, other: &Self) -> bool {
        !self.ge(other)
    }
}
