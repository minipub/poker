use std::cmp::Ordering;
use std::cmp::{PartialEq, PartialOrd};
use std::rc::Rc;

use crate::lib::card::Card;
use crate::lib::card::*;
use crate::lib::color::Color;
use crate::lib::point::Point;
use crate::lib::style::iface::*;
use crate::lib::style::CardStyle;

#[derive(Debug)]
pub struct Fools(pub [Card; 2]);

impl Fools {
    pub fn to_style(cs: &Vec<Card>) -> Option<CardStyle> {
        let mut s = Fools([Card::default(); 2]);
        let e = s.suit(&cs);
        if e.is_none() {
            return Some(CardStyle::Fools(Rc::new(s)));
        }
        None
    }
}

impl StyleCmp for Fools {
    fn cmp(&self, cs: &Vec<Card>) -> Option<CardStyle> {
        None
    }
}

impl Suit for Fools {
    type Error = &'static str;

    fn suit(&mut self, cs: &Vec<Card>) -> Option<Self::Error> {
        if cs.len() != 2 {
            return Some("Fools number must be 2.");
        }

        let g_joker = Card::new(Point::GoldenJoker(0), Color::None);
        let s_joker = Card::new(Point::SilverJoker(0), Color::None);

        if cs[0].eq(&g_joker) && cs[1].eq(&s_joker) {
            self.0[0] = cs[0];
            self.0[1] = cs[1];
        } else if cs[1].eq(&g_joker) && cs[0].eq(&s_joker) {
            self.0[0] = cs[1];
            self.0[1] = cs[0];
        } else {
            return Some("not fools.");
        }

        None
    }
}

impl PartialEq for Fools {
    fn eq(&self, other: &Self) -> bool {
        true
    }
    fn ne(&self, other: &Self) -> bool {
        false
    }
}

impl PartialOrd for Fools {
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
        false
    }

    fn ge(&self, other: &Self) -> bool {
        false
    }

    fn le(&self, other: &Self) -> bool {
        false
    }

    fn lt(&self, other: &Self) -> bool {
        false
    }
}
