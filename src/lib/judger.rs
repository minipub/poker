use rand::prelude::*;

use crate::lib::card::*;
use crate::lib::player::*;

#[derive(Debug)]
pub struct Judger {
    cards: Box<Vec<Card>>,
    lords: Box<Vec<Card>>,
}

impl Judger {
    pub fn new(vs: Box<Vec<Card>>) -> Judger {
        Judger {
            cards: vs,
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

    pub fn deal_lord(&mut self, p: &mut Player) {
        let mut next = true;
        while next {
            next = match self.lords.pop() {
                Some(t) => {
                    p.push_card(t);
                    true
                }
                None => false,
            }
        }
        p.set_lord();
    }

    // fn cut(&mut self) {
    //     let mut rng = rand::thread_rng();
    //     let x: u8 = rng.gen_range(0..(self.cards.len() as u8));

    //     let mcnt = self.cards.len() / 2;
    //     let mut i = mcnt;
    //     while i < self.cards.len() {
    //         let tmp = self.cards[i];
    //         self.cards[i] = self.cards[i - mcnt];
    //         self.cards[i - mcnt] = tmp;
    //         i += 1;
    //     }
    // }

    pub fn deal(&mut self, mut ps: [&mut Player; 3]) {
        let mut next = true;
        while next {
            for p in ps.iter_mut() {
                next = self.deal_one_card(p);
            }
        }
    }

    pub fn deal_one_card(&mut self, p: &mut Player) -> bool {
        match self.cards.pop() {
            Some(t) => {
                p.push_card(t);
                true
            }
            None => false,
        }
    }

    pub fn cards_count(&self) -> usize {
        self.cards.len()
    }

    pub fn lords_count(&self) -> usize {
        self.lords.len()
    }
}
