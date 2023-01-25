use std::cmp::Ordering;
use std::cmp::{PartialEq, PartialOrd};
use std::collections::HashMap;
use std::rc::Rc;

use crate::lib::card::Card;
use crate::lib::style::iface::*;

#[derive(Debug)]
pub struct FourWithPairs<const N: usize>(pub [Card; 4], [[Card; 2]; N]);

impl<const N: usize> Suit for FourWithPairs<N> {
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

        let mut i = 0;
        for p in pairs {
            self.1[i] = p;
            i += 1;
        }

        None
    }
}

impl<const N: usize> Layer for FourWithPairs<N> {
    type Other = FourWithPairs<N>;

    fn same_layer(&self, other: Self::Other) -> bool {
        self.1.len() == other.1.len()
    }
}

fn card_buckets(cs: &Vec<Card>) -> HashMap<u8, Box<Vec<Card>>> {
    let mut buckets: HashMap<u8, Box<Vec<Card>>> = HashMap::new();

    for x in cs.iter() {
        let k = x.unwrap_point();
        if !buckets.contains_key(&k) {
            buckets.insert(k, Box::new(vec![]));
        }
        buckets.get_mut(&k).unwrap().push(*x);
    }

    println!("buckets: {:?}", buckets);

    buckets
}

impl<const N: usize> PartialEq for FourWithPairs<N> {
    fn eq(&self, other: &Self) -> bool {
        self.0[0] == other.0[0]
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<const N: usize> PartialOrd for FourWithPairs<N> {
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
        let mut y = FourWithPairs([Card::default(); 4], [[Card::default(); 2]; 2]);
        let e = y.suit(&cs);
        assert_eq!(true, e.is_none());

        let cs: Vec<Card> = vec![
            Card::new(Point::Five(0), Color::Spades),
            Card::new(Point::Ten(0), Color::Spades),
            Card::new(Point::Ten(0), Color::Plum),
            Card::new(Point::Ten(0), Color::Square),
            Card::new(Point::Ten(0), Color::Hearts),
            Card::new(Point::Five(0), Color::Hearts),
            Card::new(Point::Six(0), Color::Hearts),
            Card::new(Point::Six(0), Color::Plum),
        ];
        let mut y = FourWithPairs([Card::default(); 4], [[Card::default(); 2]; 2]);
        let e = y.suit(&cs);
        assert_eq!(true, e.is_none());
    }
}
