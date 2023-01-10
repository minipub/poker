use crate::lib::card::*;
use crate::lib::style::iface::*;
use crate::lib::style::CardStyle;

#[derive(Debug)]
pub struct Single(pub Card);

impl Single {
    pub fn to_style(cs: &Vec<Card>) -> Option<CardStyle> {
        let mut s = Single(Card::default());
        let e = s.suit(&cs);
        if e == None {
            return Some(CardStyle::Single(s));
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
