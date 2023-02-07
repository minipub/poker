use std::cell::RefCell;
use std::cmp::PartialEq;
use std::rc::Rc;

use crate::lib::card::Card;
use crate::lib::deck;
use crate::lib::deck::{DECK_ONE, DECK_TWO};
use crate::lib::judger;
use crate::lib::player;
use crate::lib::player::PlayerNum;
use crate::lib::style;

// NumConfig(PlayerNum, DeckNum, ReserveNum)
#[derive(Debug, PartialEq)]
pub enum NumConfig {
    Three(u8, u8, u8),
    Four(u8, u8, u8),
    None,
}

#[derive(Debug)]
pub struct Session<'a> {
    id: u64,
    judger: judger::Judger,
    players: Vec<Rc<RefCell<player::Player<'a>>>>,
    round: Round<'a>,
    player_num: u8,
    deck_num: u8,
    reserve_num: u8,
}

#[derive(Debug)]
pub struct Round<'a> {
    player: Option<Rc<RefCell<player::Player<'a>>>>,
    style: Option<style::CardStyle>,
}

impl<'a> Session<'a> {
    pub fn new(id: u64, nc: NumConfig) -> Result<Session<'a>, &'a str> {
        // let (deck_num, reserve_num): (u8, u8);

        let (player_num, deck_num, reserve_num) = match nc {
            NumConfig::Three(_, _, _) => (3, DECK_ONE, 3),
            NumConfig::Four(_, _, _) => (4, DECK_TWO, 8),
            NumConfig::None => {
                return Err("only 3 or 4 players in one game.");
            }
        };

        let ds = deck::Deck::new(deck_num);
        // println!("deck card: {:?}", ds);

        let mut j = judger::Judger::new(deck_num, ds);
        j.shuffle();

        // println!("judger shuffle: {:?}", j);
        // println!("card num: {:?}", j.cards_count());

        j.reserve(reserve_num);
        // println!("judger reserve: {:?}", j);

        Ok(Session {
            id,
            judger: j,
            players: vec![],
            round: Round {
                player: None,
                style: None,
            },
            player_num,
            deck_num,
            reserve_num,
        })
    }

    pub fn push_player(&mut self, p: Rc<RefCell<player::Player<'a>>>) {
        let seat = self.players.len();
        p.borrow_mut().set_seat(seat);
        self.players.push(p);
    }

    pub fn begin(&mut self) {
        let mut ps: Vec<Rc<RefCell<player::Player>>> = vec![];

        for i in &self.players {
            ps.push(i.clone());
        }

        self.judger.deal(&ps);
    }

    pub fn set_lord(&mut self, i: usize) {
        let p = self.players.get(i).unwrap();
        self.judger.deal_lord(p.clone());
    }

    pub fn play_round(&mut self, p: Rc<RefCell<player::Player<'a>>>, cs: &Vec<Card>) -> bool {
        let mut ids: Vec<usize> = vec![];
        for c in cs {
            match p.borrow_mut().find_card(&c) {
                Ok(i) => ids.push(i),
                Err(i) => {
                    eprintln!("find card err: can't find element but before idx {:?}", i);
                    return false;
                }
            };
        }

        let now_cs: Option<style::CardStyle>;

        if self.round.style.is_none() {
            now_cs = style::CardStyle::to_style(&cs);
        } else {
            now_cs = self.round.style.as_ref().unwrap().cmp(&cs);
        }

        if now_cs.is_none() {
            return false;
        }

        self.round.style = now_cs;
        self.round.player = Some(p.clone());

        for i in ids {
            p.borrow_mut().del_card_by_idx(i);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::player::Player;

    #[test]
    fn test_session_3_players() {
        // 3 players, 1 deck of card
        let ws = Session::new(1001, NumConfig::Three(0, 0, 0));
        let mut s = match ws {
            Ok(s) => s,
            Err(e) => {
                panic!("{:?}", e);
            }
        };

        let p1 = Rc::new(RefCell::new(Player::new(100, "john")));
        let p2 = Rc::new(RefCell::new(Player::new(101, "mike")));
        let p3 = Rc::new(RefCell::new(Player::new(102, "alex")));

        s.push_player(p1);
        s.push_player(p2);
        s.push_player(p3);

        s.begin();

        // each person has 17 cards
        for p in s.players.iter() {
            assert_eq!(17, p.borrow().cards_count());
        }

        s.set_lord(2);

        println!("session: {:?}", s);

        // lord has 20 cards
        let lord = s.players.get_mut(2).unwrap();
        assert_eq!(20, lord.borrow().cards_count());
    }

    #[test]
    fn test_session_4_players() {
        // 4 players, 2 decks of cards
        let ws = Session::new(1002, NumConfig::Four(0, 0, 0));
        let mut s = match ws {
            Ok(s) => s,
            Err(e) => {
                panic!("{:?}", e);
            }
        };

        // println!("session hhh: {:?}", s);

        let p1 = Rc::new(RefCell::new(Player::new(100, "john")));
        let p2 = Rc::new(RefCell::new(Player::new(101, "mike")));
        let p3 = Rc::new(RefCell::new(Player::new(102, "alex")));
        let p4 = Rc::new(RefCell::new(Player::new(103, "bobb")));

        s.push_player(p1);
        s.push_player(p2);
        s.push_player(p3);
        s.push_player(p4);

        s.begin();

        // each person has 25 cards
        for p in s.players.iter() {
            assert_eq!(25, p.borrow().cards_count());
        }

        s.set_lord(1);

        println!("session: {:?}", s);

        // lord has 33 cards
        let lord = s.players.get_mut(1).unwrap();
        assert_eq!(33, lord.borrow().cards_count());
    }

    #[test]
    fn test_session_none_numconfig() {
        let ws = Session::new(1003, NumConfig::None);
        match ws {
            Ok(_) => {
                panic!("no this situation: NumConfig::None.");
            }
            Err(e) => {
                assert_eq!("only 3 or 4 players in one game.", e);
            }
        };
    }

    // #[test]
    // fn test_play_round() {
    //     // 3 players, 1 deck of card
    //     let ws = Session::new(1004, NumConfig::Three(0, 0, 0));
    //     let mut s = match ws {
    //         Ok(s) => s,
    //         Err(e) => {
    //             panic!("{:?}", e);
    //         }
    //     };

    //     let p1 = Rc::new(RefCell::new(Player::new(1, "john")));
    //     let p2 = Rc::new(RefCell::new(Player::new(2, "mike")));
    //     let p3 = Rc::new(RefCell::new(Player::new(3, "alex")));

    //     s.push_player(p1);
    //     s.push_player(p2);
    //     s.push_player(p3);

    //     s.begin();

    //     // each person has 17 cards
    //     for p in s.players.iter() {
    //         assert_eq!(17, p.borrow().cards_count());
    //     }

    //     s.set_lord(2);

    //     println!("session: {:?}", s);
    // }
}
