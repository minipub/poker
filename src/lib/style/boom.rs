use std::cmp::Ordering;
use std::cmp::{PartialEq, PartialOrd};
use std::rc::Rc;

use crate::lib::card::*;
use crate::lib::style::iface::*;
use crate::lib::style::CardStyle;

#[derive(Debug)]
pub struct Bomb(pub [Card; 4]);

impl Bomb {
    pub fn to_style(cs: &Vec<Card>) -> Option<CardStyle> {
        let mut s = Bomb([Card::default(); 4]);
        let e = s.suit(&cs);
        if e == None {
            return Some(CardStyle::Boom(Rc::new(s)));
        }
        None
    }
}

impl StyleCmp for Bomb {
    fn cmp(&self, cs: &Vec<Card>) -> Option<CardStyle> {
        let mut y = Bomb([Card::default(); 4]);
        let e = y.suit(&cs);
        if e.is_none() && y > *self {
            return Some(CardStyle::Boom(Rc::new(y)));
        }
        None
    }
}

impl Suit for Bomb {
    type Error = &'static str;

    fn suit(&mut self, cs: &Vec<Card>) -> Option<Self::Error> {
        if cs.len() != 4 {
            return Some("bomb number must be 4.");
        }

        if cs[0] == cs[1] && cs[1] == cs[2] && cs[2] == cs[3] {
            self.0 = [cs[0], cs[0], cs[0], cs[0]];
            None
        } else {
            return Some("bomb number elements must be equal.");
        }
    }
}

impl PartialEq for Bomb {
    fn eq(&self, other: &Self) -> bool {
        self.0[0] == other.0[0]
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Bomb {
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
