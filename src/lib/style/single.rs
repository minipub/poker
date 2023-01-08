use crate::lib::card::*;
use crate::lib::style::iface::*;

#[derive(Debug)]
pub struct Single(pub Card);

impl Suit for Single {
    type Error = &'static str;

    fn suit(&mut self, cs: &Vec<Card>) -> Option<Self::Error> {
        if cs.len() == 1 {
            None
        } else {
            Some("Single number must be 1.")
        }
    }
}
