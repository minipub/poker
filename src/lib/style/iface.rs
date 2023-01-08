use crate::lib::card::*;

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
