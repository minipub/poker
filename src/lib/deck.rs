use crate::lib::card::*;
use crate::lib::color::*;
use crate::lib::point::*;

pub const DECK_ONE: u8 = 1;
pub const DECK_TWO: u8 = 2;

/// Deck of Cards
#[derive(Debug)]
pub struct Deck {}

impl Deck {
    pub fn new(cnt: u8) -> Box<Vec<Card>> {
        let points = [
            Point::BigTwo(0),
            Point::Ace(0),
            Point::King(0),
            Point::Queen(0),
            Point::Jack(0),
            Point::Ten(0),
            Point::Nine(0),
            Point::Eight(0),
            Point::Seven(0),
            Point::Six(0),
            Point::Five(0),
            Point::Four(0),
            Point::Three(0),
        ];
        let colors = [Color::Spades, Color::Plum, Color::Square, Color::Hearts];

        let mut vs: Vec<Card> = Vec::new();

        let mut i = 0;

        while i < cnt {
            i += 1;

            vs.push(Card::new(Point::GoldenJoker(0), Color::None));
            vs.push(Card::new(Point::SilverJoker(0), Color::None));

            for p in points {
                for c in colors {
                    let t = Card::new(p, c);
                    vs.push(t);
                }
            }
        }

        Box::new(vs)
    }
}
