use crate::lib::card::*;
use crate::lib::style::iface::*;
use crate::lib::style::CardStyle;
use std::rc::Rc;

#[derive(Debug)]
pub struct Single(pub Card);

impl Single {
    pub fn to_style(cs: &Vec<Card>) -> Option<CardStyle> {
        let mut s = Single(Card::default());
        let e = s.suit(&cs);
        if e == None {
            return Some(CardStyle::Single(Rc::new(s)));
        }
        None
    }
}

impl StyleCmp for Single {
    fn cmp(&self, cs: &Vec<Card>) -> Option<CardStyle> {
        let mut y = Single(Card::default());
        let e = y.suit(&cs);
        if e.is_none() {
            if y.0 > self.0 {
                return Some(CardStyle::Single(Rc::new(y)));
            } else {
                return None;
            }
        }
        None
    }
}

impl Suit for Single {
    type Error = &'static str;

    fn suit(&mut self, cs: &Vec<Card>) -> Option<Self::Error> {
        if cs.len() == 1 {
            self.0 = cs[0];
            None
        } else {
            Some("Single number must be 1.")
        }
    }
}
