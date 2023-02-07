use std::cell::RefCell;
use std::rc::Rc;

use crate::lib::card::Card;

#[derive(Debug, PartialEq)]
pub enum PlayerNum {
    Three(u8),
    Four(u8),
}

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

    pub fn push_card_func(p: Rc<RefCell<Player>>, is_diff: bool) -> Box<dyn FnMut(Card) + '_> {
        if is_diff {
            Box::new(move |c| p.borrow_mut().push_diff_card(c))
        } else {
            Box::new(move |c| p.borrow_mut().push_dup_card(c))
        }
    }

    pub fn push_diff_card(&mut self, t: Card) {
        // keep the cards in ascending order
        let mut cm = self.cards.borrow_mut();
        match cm.binary_search_by(|pb| pb.partial_cmp(&t).unwrap()) {
            Ok(idx) => {
                eprintln!(
                    "push_card err: shouldn't find element idx {:?}, now_cards: {:?}, c: {:?}",
                    idx, cm, t,
                );
                return;
            }
            Err(idx) => {
                cm.insert(idx, t);
                return;
            }
        };
    }

    // push dup cards when two of them may be equal
    // two cards would be equal in two deck of cards
    pub fn push_dup_card(&mut self, t: Card) {
        let mut cm = self.cards.borrow_mut();
        match cm.binary_search_by(|pb| pb.partial_cmp(&t).unwrap()) {
            Ok(idx) => {
                cm.insert(idx, t);
                return;
            }
            Err(idx) => {
                cm.insert(idx, t);
                return;
            }
        };
    }

    pub fn find_card(&mut self, t: &Card) -> Result<usize, usize> {
        self.cards
            .borrow_mut()
            .binary_search_by(|pb| pb.partial_cmp(t).unwrap())
    }

    pub fn del_card_by_idx(&mut self, idx: usize) {
        self.cards.borrow_mut().remove(idx);
    }

    pub fn del_card(&mut self, t: &Card) {
        let mut cm = self.cards.borrow_mut();
        match cm.binary_search_by(|pb| pb.partial_cmp(t).unwrap()) {
            Ok(idx) => cm.remove(idx),
            Err(idx) => {
                eprintln!("del_card err: can't find element but before idx {:?}", idx);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::color::Color;
    use crate::lib::point::Point;

    #[test]
    fn test_push_card() {
        let mut p1 = Player::new(1, "john");
        let cs: Vec<Card> = vec![
            Card::new(Point::Jack(0), Color::Plum),
            Card::new(Point::Queen(0), Color::Spades),
            Card::new(Point::Eight(0), Color::Square),
            Card::new(Point::Three(0), Color::Hearts),
            Card::new(Point::Four(0), Color::Square),
            Card::new(Point::Seven(0), Color::Square),
            Card::new(Point::GoldenJoker(0), Color::None),
            Card::new(Point::Seven(0), Color::Spades),
            Card::new(Point::Seven(0), Color::Plum),
            Card::new(Point::Five(0), Color::Hearts),
            Card::new(Point::SilverJoker(0), Color::None),
        ];
        for c in &cs {
            p1.push_diff_card(*c);
        }
        println!("p1: {:?}", p1);
        assert_eq!(cs[3], *(p1.cards.borrow().get(0).unwrap()));
        assert_eq!(cs[7], *(p1.cards.borrow().get(3).unwrap()));
        assert_eq!(cs[8], *(p1.cards.borrow().get(4).unwrap()));
        assert_eq!(cs[5], *(p1.cards.borrow().get(5).unwrap()));
        assert_eq!(cs[6], *(p1.cards.borrow().get(10).unwrap()));
    }

    #[test]
    fn test_del_card() {
        let mut p1 = Player::new(1, "john");
        let cs: Vec<Card> = vec![
            Card::new(Point::Jack(0), Color::Plum),
            Card::new(Point::Queen(0), Color::Spades),
            Card::new(Point::Eight(0), Color::Square),
            Card::new(Point::Three(0), Color::Hearts),
            Card::new(Point::Four(0), Color::Square),
            Card::new(Point::Seven(0), Color::Square),
            Card::new(Point::GoldenJoker(0), Color::None),
            Card::new(Point::Seven(0), Color::Spades),
            Card::new(Point::Seven(0), Color::Plum),
            Card::new(Point::Five(0), Color::Hearts),
            Card::new(Point::SilverJoker(0), Color::None),
        ];
        for c in &cs {
            p1.push_diff_card(*c);
        }

        p1.del_card(&Card::new(Point::Five(0), Color::Hearts));
        assert_eq!(cs[7], *(p1.cards.borrow().get(2).unwrap()));

        // println!("p1: {:?}", p1);
        // println!("cs: {:?}", cs);
        p1.del_card(&Card::new(Point::Seven(0), Color::Plum));
        assert_eq!(cs[2], *(p1.cards.borrow().get(4).unwrap()));
    }
}
