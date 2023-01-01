use crate::lib::card::*;

#[derive(Debug)]
pub struct Player<'a> {
    id: u8,
    name: &'a str,
    is_lord: bool,
    cards: Box<Vec<Card>>,
}

impl<'a> Player<'a> {
    pub fn new(id: u8, name: &str) -> Player {
        Player {
            id,
            name,
            is_lord: false,
            cards: Box::new(vec![]),
        }
    }

    pub fn set_lord(&mut self) {
        self.is_lord = true;
    }

    pub fn push_card(&mut self, t: Card) {
        self.cards.push(t);
    }

    pub fn cards_count(&self) -> usize {
        self.cards.len()
    }
}
