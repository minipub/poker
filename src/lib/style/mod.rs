pub mod boom;
pub mod chain;
pub mod fools;
pub mod four_with_pairs;
pub mod four_with_two;
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
    FourWithPairs(Rc<four_with_pairs::FourWithPairs>),
    FourWithTwo(Rc<four_with_two::FourWithTwo>),
    Boom(Rc<boom::Bomb>),
    Chain(Rc<chain::Chain>),
    Pairs(Rc<pairs::Pairs>),
    ThreeWithOnes(Rc<three_with_ones::ThreeWithOnes>),
    ThreeWithPairs(Rc<three_with_pairs::ThreeWithPairs>),
    Threes(Rc<threes::Threes>),
    Single(Rc<single::Single>),
}

impl CardStyle {
    pub fn cmp(&self, cs: &Vec<Card>) -> Option<CardStyle> {
        match self {
            CardStyle::Fools(x) => x.cmp(&cs),

            CardStyle::FourWithPairs(x) => x.cmp(&cs),

            CardStyle::FourWithTwo(x) => x.cmp(&cs),

            CardStyle::Boom(x) => x.cmp(&cs),

            CardStyle::Chain(x) => x.cmp(&cs),

            CardStyle::Pairs(x) => x.cmp(&cs),

            CardStyle::ThreeWithOnes(x) => x.cmp(&cs),

            CardStyle::ThreeWithPairs(x) => x.cmp(&cs),

            CardStyle::Threes(x) => x.cmp(&cs),

            CardStyle::Single(x) => x.cmp(&cs),
        }
    }

    pub fn to_style(cs: &Vec<Card>) -> Option<CardStyle> {
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

        let a = match CardStyle::to_style(&vec![t]) {
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

        let a = match CardStyle::to_style(&vec![t1, t2, t3, t4, t5]) {
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

    #[test]
    fn test_cmp() {
        {
            let t0 = Card::new(Point::Nine(0), Color::Spades);
            let t1 = Card::new(Point::Ten(0), Color::Spades);
            let t2 = Card::new(Point::King(0), Color::Plum);
            let t3 = Card::new(Point::Jack(0), Color::Square);
            let t4 = Card::new(Point::Queen(0), Color::Square);
            let t5 = Card::new(Point::Ace(0), Color::Hearts);

            let cs = vec![t0, t1, t2, t3, t4, t5];

            let x = CardStyle::Chain(Rc::new(chain::Chain(vec![t0, t1, t2, t3, t4])));

            let y = x.cmp(&cs);

            assert_eq!(true, y.is_none());
        }

        {
            let t0 = Card::new(Point::Nine(0), Color::Spades);
            let t1 = Card::new(Point::Ten(0), Color::Spades);
            let t2 = Card::new(Point::King(0), Color::Plum);
            let t3 = Card::new(Point::Jack(0), Color::Square);
            let t4 = Card::new(Point::Queen(0), Color::Square);
            let t5 = Card::new(Point::Ace(0), Color::Hearts);

            let cs = vec![t1, t2, t3, t4, t5];

            let x = CardStyle::Chain(Rc::new(chain::Chain(vec![t0, t1, t2, t3, t4])));

            let y = x.cmp(&cs);

            assert_eq!(true, y.is_some());
        }
    }
}
