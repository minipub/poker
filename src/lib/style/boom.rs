use std::cmp::Ordering;
use std::cmp::{PartialEq, PartialOrd};

use crate::lib::card::*;
use crate::lib::style::iface::Suit;

#[derive(Debug)]
struct Bomb(Box<[Card; 4]>);

impl Suit for Bomb {
    type Error = &'static str;

    fn suit(&mut self, cs: Box<Vec<Card>>) -> Option<Self::Error> {
        if cs.len() != 4 {
            return Some("bomb number must be 4.");
        }

        if cs[0] == cs[1] && cs[1] == cs[2] && cs[2] == cs[3] {
            self.0 = Box::new([cs[0], cs[0], cs[0], cs[0]]);
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
