use std::cmp::PartialEq;

use crate::lib::deck;
use crate::lib::judger;
use crate::lib::player;
use crate::lib::style;

#[derive(Debug, PartialEq)]
pub enum SessionType {
    Three,
    Four,
}

#[derive(Debug)]
pub struct Session<'a> {
    id: u64,
    judger: judger::Judger,
    players: Vec<player::Player<'a>>,
    round: Option<style::CardStyle>,
    stype: SessionType,
}

impl<'a> Session<'a> {
    pub fn new(id: u64, decknum: u8, stype: SessionType) -> Session<'a> {
        let ds = deck::Deck::new(decknum);
        // println!("deck card: {:?}", ds);

        let mut j = judger::Judger::new(ds);
        j.shuffle();

        // println!("judger shuffle: {:?}", j);
        // println!("card num: {:?}", j.cards_count());

        j.reserve(3);
        // println!("judger reserve: {:?}", j);

        Session {
            id,
            judger: j,
            players: vec![],
            round: None,
            stype,
        }
    }

    pub fn push_player(&mut self, mut p: player::Player<'a>) {
        let seat = self.players.len();
        p.set_seat(seat);
        self.players.push(p);
    }

    pub fn begin(&mut self) -> bool {
        if self.stype == SessionType::Three {
            if self.players.len() != 3 {
                return false;
            }

            let mut ps: Vec<&mut player::Player> = vec![];
            for i in self.players.iter_mut() {
                // let mut p = i;
                ps.push(i)
            }
            // let mut p1 = self.players.get_mut(0).unwrap();
            // let mut p2 = self.players.get_mut(1).unwrap();
            // let mut p3 = self.players.get_mut(2).unwrap();

            // self.judger.deal([&mut p1, &mut p2, &mut p3]);
            self.judger.deal(ps);
        }
        false
    }

    pub fn set_lord(&mut self, i: usize) {
        let p = self.players.get_mut(i).unwrap();
        self.judger.deal_lord(p);
    }
}
