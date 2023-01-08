use crate::lib::card::*;

#[derive(Debug)]
pub struct Player<'a> {
    id: u8,
    name: &'a str,
    is_lord: bool,
    cards: Box<Vec<Card>>,
    seat: i8,
}

impl<'a> Player<'a> {
    pub fn new(id: u8, name: &str) -> Player {
        Player {
            id,
            name,
            is_lord: false,
            cards: Box::new(vec![]),
            seat: -1,
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

    pub fn set_seat(&mut self, i: usize) {
        self.seat = i as i8;
    }

    pub fn play(cs: Box<Vec<Card>>) {}
}
