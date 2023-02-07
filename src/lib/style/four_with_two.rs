use std::cmp::Ordering;
use std::cmp::{PartialEq, PartialOrd};
use std::rc::Rc;

use crate::lib::card::Card;
use crate::lib::style::iface::*;
use crate::lib::style::CardStyle;
use crate::lib::util::card_buckets;

#[derive(Debug)]
pub struct FourWithTwo(pub [Card; 4], Vec<Card>);

impl FourWithTwo {
    pub fn to_style(cs: &Vec<Card>) -> Option<CardStyle> {
        let mut s = FourWithTwo([Card::default(); 4], vec![Card::default()]);
        let e = s.suit(&cs);
        if e == None {
            return Some(CardStyle::FourWithTwo(Rc::new(s)));
        }
        None
    }
}

impl StyleCmp for FourWithTwo {
    fn cmp(&self, cs: &Vec<Card>) -> Option<CardStyle> {
        let mut y = FourWithTwo([Card::default(); 4], vec![Card::default()]);
        let e = y.suit(&cs);
        if e.is_none() {
            if y > *(self) {
                return Some(CardStyle::FourWithTwo(Rc::new(y)));
            } else {
                return None;
            }
        }
        None
    }
}

impl Suit for FourWithTwo {
    type Error = &'static str;

    fn suit(&mut self, cs: &Vec<Card>) -> Option<Self::Error> {
        if cs.len() != 6 {
            return Some("FourWithTwo number must be 6.");
        }

        let buckets = card_buckets(cs);

        if buckets.len() != 3 {
            return Some("FourWithTwo buckets must be 3.");
        }

        let mut four = [Card::default(); 4];
        let mut two: Vec<Card> = vec![];

        for (k, v) in buckets {
            if v.len() == 4 {
                (four[0], four[1], four[2], four[3]) = (v[0], v[1], v[2], v[3]);
            } else if v.len() == 1 {
                two.push(v[0]);
            } else {
                return Some("FourWithTwo must be consist of 4 and 2.");
            }
        }

        if two.len() != 2 {
            return Some("FourWithTwo two number must be 2.");
        }

        if two[0] == two[1] {
            return Some("FourWithTwo two elements must be not equal.");
        }

        self.0 = four;
        self.1 = two;

        None
    }
}

impl PartialEq for FourWithTwo {
    fn eq(&self, other: &Self) -> bool {
        self.0[0] == other.0[0]
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for FourWithTwo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.gt(other) {
            Some(Ordering::Greater)
        } else if self.lt(other) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }

    fn gt(&self, other: &Self) -> bool {
        self.0[0] > other.0[0]
    }

    fn ge(&self, other: &Self) -> bool {
        self.0[0] >= other.0[0]
    }

    fn le(&self, other: &Self) -> bool {
        !self.gt(other)
    }

    fn lt(&self, other: &Self) -> bool {
        !self.ge(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::color::Color;
    use crate::lib::point::Point;

    #[test]
    fn test_suit() {
        let cs: Vec<Card> = vec![
            Card::new(Point::Five(0), Color::Spades),
            Card::new(Point::Ten(0), Color::Spades),
            Card::new(Point::Ten(0), Color::Plum),
            Card::new(Point::Ten(0), Color::Square),
            Card::new(Point::Ten(0), Color::Hearts),
            Card::new(Point::Five(0), Color::Hearts),
        ];
        let mut y1 = FourWithTwo([Card::default(); 4], vec![]);
        let e = y1.suit(&cs);
        assert_eq!(false, e.is_none());

        let cs: Vec<Card> = vec![
            Card::new(Point::Five(0), Color::Spades),
            Card::new(Point::Nine(0), Color::Spades),
            Card::new(Point::Nine(0), Color::Plum),
            Card::new(Point::Nine(0), Color::Square),
            Card::new(Point::Nine(0), Color::Hearts),
            Card::new(Point::Five(0), Color::Hearts),
            Card::new(Point::Six(0), Color::Hearts),
            Card::new(Point::Six(0), Color::Plum),
        ];
        let mut y2 = FourWithTwo([Card::default(); 4], vec![]);
        let e = y2.suit(&cs);
        assert_eq!(false, e.is_none());

        let cs: Vec<Card> = vec![
            Card::new(Point::Ace(0), Color::Spades),
            Card::new(Point::Three(0), Color::Spades),
            Card::new(Point::Three(0), Color::Plum),
            Card::new(Point::Seven(0), Color::Plum),
            Card::new(Point::Three(0), Color::Square),
            Card::new(Point::Three(0), Color::Hearts),
        ];
        let mut y3 = FourWithTwo([Card::default(); 4], vec![]);
        let e = y3.suit(&cs);
        println!("e: {:?}", e);
        assert_eq!(true, e.is_none());
    }
}
