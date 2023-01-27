use std::cell::RefCell;
use std::rc::Rc;

use crate::lib::card::*;

#[derive(Debug)]
pub struct Player<'a> {
    id: u8,
    name: &'a str,
    is_lord: bool,
    cards: Rc<RefCell<Vec<Card>>>,
    seat: i8,
}

impl<'a> Player<'a> {
    pub fn new(id: u8, name: &str) -> Player {
        Player {
            id,
            name,
            is_lord: false,
            cards: Rc::new(RefCell::new(vec![])),
            seat: -1,
        }
    }

    pub fn set_lord(&mut self) {
        self.is_lord = true;
    }

    pub fn push_card(&mut self, t: Card) {
        // TODO keep the cards in ascending order
        self.cards.borrow_mut().push(t);
    }

    pub fn del_card(&mut self, t: Card) {
        match self
            .cards
            .borrow_mut()
            .binary_search_by(|pb| pb.partial_cmp(&t).unwrap())
        {
            Ok(idx) => self.cards.borrow_mut().remove(idx),
            Err(_) => {
                return;
            }
        };
    }

    pub fn cards_count(&self) -> usize {
        self.cards.borrow().len()
    }

    pub fn set_seat(&mut self, i: usize) {
        self.seat = i as i8;
    }

    pub fn play(cs: Box<Vec<Card>>) {}
}
