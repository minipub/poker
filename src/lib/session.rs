use std::cmp::PartialEq;

use crate::lib::card::Card;
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
    round: Round<'a>,
    stype: SessionType,
}

#[derive(Debug)]
pub struct Round<'a> {
    player: Option<player::Player<'a>>,
    style: Option<style::CardStyle>,
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
            round: Round {
                player: None,
                style: None,
            },
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

            true
        } else {
            false
        }
    }

    pub fn set_lord(&mut self, i: usize) {
        let p = self.players.get_mut(i).unwrap();
        self.judger.deal_lord(p);
    }

    pub fn play_round(&mut self, p: player::Player<'a>, cs: &Vec<Card>) {
        let now_cs = style::CardStyle::cmp(&self.round.style, &cs);
        if now_cs.is_none() {
            return;
        }

        self.round.style = now_cs;
        self.round.player = Some(p);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::player::Player;

    #[test]
    fn test_session() {
        let mut s = Session::new(1001, 1, SessionType::Three);

        let mut p1 = Player::new(100, "john");
        let mut p2 = Player::new(101, "mike");
        let mut p3 = Player::new(102, "alex");

        s.push_player(p1);
        s.push_player(p2);
        s.push_player(p3);

        let started = s.begin();

        // each person has 17 cards
        for p in s.players.iter() {
            assert_eq!(17, p.cards_count());
        }

        assert_eq!(true, started);

        if started {
            s.set_lord(2);
        }

        println!("session: {:?}", s);

        let lord = s.players.get_mut(2).unwrap();
        assert_eq!(20, lord.cards_count());
    }
}
