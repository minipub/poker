use std::cmp::Ordering;
use std::cmp::{PartialEq, PartialOrd};
use std::rc::Rc;

use crate::lib::card::Card;
use crate::lib::style::iface::*;
use crate::lib::style::CardStyle;
use crate::lib::util::card_buckets;

#[derive(Debug)]
pub struct FourWithPairs(pub [Card; 4], Vec<[Card; 2]>);

impl FourWithPairs {
    pub fn to_style(cs: &Vec<Card>) -> Option<CardStyle> {
        let mut s = FourWithPairs([Card::default(); 4], vec![[Card::default(); 2]]);
        let e = s.suit(&cs);
        if e == None {
            return Some(CardStyle::FourWithPairs(Rc::new(s)));
        }
        None
    }
}

impl StyleCmp for FourWithPairs {
    fn cmp(&self, cs: &Vec<Card>) -> Option<CardStyle> {
        let mut y = FourWithPairs([Card::default(); 4], vec![[Card::default(); 2]]);
        let e = y.suit(&cs);
        if e.is_none() {
            if y > *(self) {
                return Some(CardStyle::FourWithPairs(Rc::new(y)));
            } else {
                return None;
            }
        }
        None
    }
}

impl Suit for FourWithPairs {
    type Error = &'static str;

    fn suit(&mut self, cs: &Vec<Card>) -> Option<Self::Error> {
        if cs.len() != 6 && cs.len() != 8 {
            return Some("FourWithPairs number must be 6 or 8.");
        }

        let buckets = card_buckets(cs);

        if buckets.len() != 2 && buckets.len() != 3 {
            return Some("FourWithPairs buckets must be 2 or 3.");
        }

        let mut four = [Card::default(); 4];
        let mut pairs: Vec<[Card; 2]> = vec![];

        for (k, v) in buckets {
            if v.len() == 4 {
                (four[0], four[1], four[2], four[3]) = (v[0], v[1], v[2], v[3]);
            } else if v.len() == 2 {
                if v[0] == v[1] {
                    pairs.push([v[0], v[1]]);
                } else {
                    return Some("FourWithPairs pairs item must equal.");
                }
            } else {
                return Some("FourWithPairs must be consist of 4 and 2s.");
            }
        }

        if pairs.len() == 0 || pairs.len() > 2 {
            return Some("FourWithPairs pairs number be 1 or 2.");
        }

        self.0 = four;
        self.1 = pairs;

        None
    }
}

impl Layer for &FourWithPairs {
    type Other = Self;

    fn same_layer(&self, other: Self::Other) -> bool {
        self.1.len() == other.1.len()
    }
}

impl PartialEq for FourWithPairs {
    fn eq(&self, other: &Self) -> bool {
        self.0[0] == other.0[0]
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for FourWithPairs {
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
        let mut y1 = FourWithPairs([Card::default(); 4], vec![[Card::default(); 2]]);
        let e = y1.suit(&cs);
        assert_eq!(true, e.is_none());

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
        let mut y2 = FourWithPairs([Card::default(); 4], vec![[Card::default(); 2]]);
        let e = y2.suit(&cs);
        assert_eq!(true, e.is_none());

        let cs: Vec<Card> = vec![
            Card::new(Point::Ace(0), Color::Spades),
            Card::new(Point::Three(0), Color::Spades),
            Card::new(Point::Seven(0), Color::Hearts),
            Card::new(Point::Three(0), Color::Plum),
            Card::new(Point::Ace(0), Color::Hearts),
            Card::new(Point::Seven(0), Color::Plum),
            Card::new(Point::Three(0), Color::Square),
            Card::new(Point::Three(0), Color::Hearts),
        ];
        let mut y3 = FourWithPairs([Card::default(); 4], vec![[Card::default(); 2]]);
        let e = y3.suit(&cs);
        assert_eq!(true, e.is_none());

        assert_eq!(false, (&y1).same_layer(&y2));
        assert_eq!(true, (&y3).same_layer(&y2));
    }
}
