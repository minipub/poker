use crate::lib::card::*;

pub trait Suit {
    type Error;
    fn suit(&mut self, _: Box<Vec<Card>>) -> Option<Self::Error>;
}
