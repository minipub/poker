use crate::lib::card::*;
use crate::lib::style::CardStyle;

pub trait Suit {
    type Error;
    fn suit(&mut self, _: &Vec<Card>) -> Option<Self::Error>;
}

pub trait Layer {
    type Other;
    fn same_layer(&self, _: Self::Other) -> bool {
        true
    }
}

pub trait StyleCmp {
    fn cmp(&self, cs: &Vec<Card>) -> Option<CardStyle>;
}

pub type ToStyle = fn(cs: &Vec<Card>) -> Option<CardStyle>;
