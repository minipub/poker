use rand::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::lib::card::*;
use crate::lib::deck::{DECK_ONE, DECK_TWO};
use crate::lib::player::*;

#[derive(Debug)]
pub struct Judger {
    deck_num: u8,
    cards: Box<Vec<Card>>,
    lords: Box<Vec<Card>>,
}

impl Judger {
    pub fn new(deck_num: u8, cards: Box<Vec<Card>>) -> Judger {
        Judger {
            deck_num,
            cards,
            lords: Box::new(vec![]),
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn reserve(&mut self, cnt: u8) {
        let mut i = 0;

        while i < cnt {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0..self.cards.len());
            let t = self.cards.remove(x);
            self.lords.push(t);
            i += 1;
        }
    }

    pub fn deal_lord(&mut self, p: Rc<RefCell<Player>>) {
        let mut push_card = Player::push_card_func(p.clone(), self.deck_num == DECK_ONE);
        let mut next = true;

        while next {
            next = match self.lords.pop() {
                Some(t) => {
                    push_card(t);
                    true
                }
                None => false,
            }
        }

        p.borrow_mut().set_lord();
    }

    pub fn deal(&mut self, ps: &Vec<Rc<RefCell<Player>>>) {
        let is_diff_push = self.deck_num == DECK_ONE;
        let mut next = true;

        let mut pfs: Vec<Box<dyn FnMut(Card)>> = vec![];
        for p in ps {
            let push_card = Player::push_card_func(p.clone(), is_diff_push);
            pfs.push(push_card);
        }

        while next {
            let mut i: usize = 0;
            for _p in ps {
                next = match self.cards.pop() {
                    Some(t) => {
                        pfs[i](t);
                        i += 1;
                        true
                    }
                    None => false,
                };
            }
        }
    }

    pub fn cards_count(&self) -> usize {
        self.cards.len()
    }

    pub fn lords_count(&self) -> usize {
        self.lords.len()
    }
}
