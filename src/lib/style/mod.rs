pub mod boom;
pub mod chain;
pub mod fools;
pub mod iface;
pub mod pairs;
pub mod single;
pub mod three_with_ones;
pub mod three_with_pairs;
pub mod threes;

use std::rc::Rc;

use crate::lib::card::*;
use crate::lib::style::iface::*;

#[derive(Debug)]
pub enum CardStyle {
    Fools(Rc<fools::Fools>),
    Boom(Rc<boom::Bomb>),
    Chain(Rc<chain::Chain>),
    Pairs(Rc<pairs::Pairs>),
    ThreeWithOnes(Rc<three_with_ones::ThreeWithOnes>),
    ThreeWithPairs(Rc<three_with_pairs::ThreeWithPairs>),
    Threes(Rc<threes::Threes>),
    Single(Rc<single::Single>),
}

impl CardStyle {
    pub fn cmp(round: &Option<CardStyle>, cs: &Vec<Card>) -> Option<CardStyle> {
        match round {
            Some(CardStyle::Fools(_)) => {
                return None;
            }

            Some(CardStyle::Boom(x)) => {
                let mut y = boom::Bomb([Card::default(); 4]);
                let e = y.suit(&cs);
                if e.is_none() && y > *(x.as_ref()) {
                    return Some(CardStyle::Boom(Rc::new(y)));
                }
            }

            Some(CardStyle::Chain(x)) => {
                let mut y = chain::Chain(vec![]);
                let e = y.suit(&cs);
                if e.is_none() {
                    if y > *(x.as_ref()) {
                        return Some(CardStyle::Chain(Rc::new(y)));
                    } else {
                        return None;
                    }
                }
            }

            Some(CardStyle::Pairs(x)) => {
                let mut y = pairs::Pairs(vec![]);
                let e = y.suit(&cs);
                if e.is_none() {
                    if y > *(x.as_ref()) {
                        return Some(CardStyle::Pairs(Rc::new(y)));
                    } else {
                        return None;
                    }
                }
            }

            Some(CardStyle::ThreeWithOnes(x)) => {
                let mut y = three_with_ones::ThreeWithOnes(vec![]);
                let e = y.suit(&cs);
                if e.is_none() {
                    if y > *(x.as_ref()) {
                        return Some(CardStyle::ThreeWithOnes(Rc::new(y)));
                    } else {
                        return None;
                    }
                }
            }

            Some(CardStyle::ThreeWithPairs(x)) => {
                let mut y = three_with_pairs::ThreeWithPairs(vec![]);
                let e = y.suit(&cs);
                if e.is_none() {
                    if y > *(x.as_ref()) {
                        return Some(CardStyle::ThreeWithPairs(Rc::new(y)));
                    } else {
                        return None;
                    }
                }
            }

            Some(CardStyle::Threes(x)) => {
                let mut y = threes::Threes(vec![]);
                let e = y.suit(&cs);
                if e.is_none() {
                    if y > *(x.as_ref()) {
                        return Some(CardStyle::Threes(Rc::new(y)));
                    } else {
                        return None;
                    }
                }
            }

            Some(CardStyle::Single(x)) => {
                let mut y = single::Single(Card::default());
                let e = y.suit(&cs);
                if e.is_none() {
                    if y.0 > x.0 {
                        return Some(CardStyle::Single(Rc::new(y)));
                    } else {
                        return None;
                    }
                }
            }

            None => {
                return CardStyle::unwrap(&cs);
            }
        }

        {
            let mut y = boom::Bomb([Card::default(); 4]);
            let e = y.suit(&cs);
            if e.is_none() {
                return Some(CardStyle::Boom(Rc::new(y)));
            }
        }

        None
    }

    pub fn unwrap(cs: &Vec<Card>) -> Option<CardStyle> {
        let tss: Vec<ToStyle> = vec![
            boom::Bomb::to_style,
            chain::Chain::to_style,
            pairs::Pairs::to_style,
            three_with_ones::ThreeWithOnes::to_style,
            three_with_pairs::ThreeWithPairs::to_style,
            threes::Threes::to_style,
            single::Single::to_style,
        ];

        for f in tss {
            let s = f(&cs);
            if s.is_some() {
                return s;
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

        let a = match CardStyle::unwrap(&vec![t]) {
            Some(CardStyle::Single(x)) => x.0.unwrap_point(),
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

        let a = match CardStyle::unwrap(&vec![t1, t2, t3, t4, t5]) {
            Some(CardStyle::Chain(x)) => x.as_ref().0.clone(),
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
