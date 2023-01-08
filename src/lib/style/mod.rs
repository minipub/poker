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
        // {
        //     let mut s = boom::Bomb(Box::new([Card::default(); 4]));
        //     let e = s.suit(&cs);
        //     if e == None {
        //         return Some(CardStyle::Boom(s));
        //     }
        // }
        // {
        //     let mut s = chain::Chain(Box::new(vec![]));
        //     let e = s.suit(&cs);
        //     if e == None {
        //         return Some(CardStyle::Chain(s));
        //     }
        // }

        // let mut s: Box<dyn Suit<Error = &'static str>> = Box::new(boom::Bomb([Card::default(); 4]));

        let mut vs: Vec<Box<dyn Suit<Error = &'static str>>> = vec![];
        vs.push(Box::new(boom::Bomb([Card::default(); 4])));
        vs.push(Box::new(chain::Chain(Box::new(vec![]))));

        // for s in vs {
        //     let e = s.suit(&cs);
        //     if e == None {
        //         return Some(CardStyle::Chain(s));
        //     }
        // }

        None
    }
}
