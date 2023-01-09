pub mod boom;
pub mod chain;
pub mod iface;
pub mod pairs;
pub mod single;
pub mod three_with_ones;
pub mod three_with_pairs;
pub mod threes;

use crate::lib::card::*;
use crate::lib::style::iface::*;

#[derive(Debug)]
pub enum CardStyle {
    Boom(boom::Bomb),
    Chain(chain::Chain),
    Pairs(pairs::Pairs),
    ThreeWithOnes(three_with_ones::ThreeWithOnes),
    ThreeWithPairs(three_with_pairs::ThreeWithPairs),
    Threes(threes::Threes),
    Single(single::Single),
}

impl CardStyle {
    // pub fn cmp(round: Option<CardStyle>, cs: Box<Vec<Card>>) {
    //     match round {
    //         Some(CardStyle::Boom(x)) => {}
    //         Some(CardStyle::Chain(x)) => {
    //             let mut pc = chain::Chain(Box::new(vec![]));
    //             let ce = pc.suit(cs);
    //         }
    //         Some(CardStyle::Pairs(x)) => {}
    //         Some(CardStyle::ThreeWithOnes(x)) => {}
    //         Some(CardStyle::ThreeWithPairs(x)) => {}
    //         Some(CardStyle::Threes(x)) => {}
    //         Some(CardStyle::Single(x)) => {}
    //         None => {}
    //     }
    // }

    pub fn unwrap(cs: Vec<Card>) -> Option<CardStyle> {
        {
            let mut s = boom::Bomb([Card::default(); 4]);
            let e = s.suit(&cs);
            if e == None {
                return Some(CardStyle::Boom(s));
            }
        }
        {
            let mut s = chain::Chain(Box::new(vec![]));
            let e = s.suit(&cs);
            if e == None {
                return Some(CardStyle::Chain(s));
            }
        }
        {
            let mut s = pairs::Pairs(vec![]);
            let e = s.suit(&cs);
            if e == None {
                return Some(CardStyle::Pairs(s));
            }
        }
        {
            let mut s = three_with_ones::ThreeWithOnes(vec![]);
            let e = s.suit(&cs);
            if e == None {
                return Some(CardStyle::ThreeWithOnes(s));
            }
        }
        {
            let mut s = three_with_pairs::ThreeWithPairs(vec![]);
            let e = s.suit(&cs);
            if e == None {
                return Some(CardStyle::ThreeWithPairs(s));
            }
        }
        {
            let mut s = threes::Threes(vec![]);
            let e = s.suit(&cs);
            if e == None {
                return Some(CardStyle::Threes(s));
            }
        }
        {
            let mut s = single::Single(Card::default());
            let e = s.suit(&cs);
            if e == None {
                return Some(CardStyle::Single(s));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::card::Card;
    use crate::lib::color::Color;
    use crate::lib::point::Point;
    use crate::lib::style::CardStyle;

    #[test]
    fn test_single() {
        let t = Card::new(Point::Ten(0), Color::Spades);

        let a = match CardStyle::unwrap(vec![t]) {
            Some(CardStyle::Single(single::Single(x))) => x.unwrap_point(),
            _ => {
                panic!("not single");
            }
        };

        assert_eq!(10, a);
    }

    #[test]
    fn test_chain() {
        let t1 = Card::new(Point::Ten(0), Color::Spades);
        let t2 = Card::new(Point::King(0), Color::Plum);
        let t3 = Card::new(Point::Jack(0), Color::Square);
        let t4 = Card::new(Point::Queen(0), Color::Square);
        let t5 = Card::new(Point::Ace(0), Color::Hearts);

        let a = match CardStyle::unwrap(vec![t1, t2, t3, t4, t5]) {
            Some(CardStyle::Chain(chain::Chain(x))) => x,
            _ => {
                panic!("not chain");
            }
        };

        assert_eq!(10, a[0].unwrap_point());
        assert_eq!(11, a[1].unwrap_point());
        assert_eq!(12, a[2].unwrap_point());
        assert_eq!(13, a[3].unwrap_point());
        assert_eq!(14, a[4].unwrap_point());
    }
}
