use std::cmp::Ordering;
use std::cmp::{PartialEq, PartialOrd};

type GoldenJoker = u8;
type SilverJoker = u8;
type BigTwo = u8;
type Ace = u8;
type King = u8;
type Queen = u8;
type Jack = u8;
type Ten = u8;
type Nine = u8;
type Eight = u8;
type Seven = u8;
type Six = u8;
type Five = u8;
type Four = u8;
type Three = u8;

#[derive(Debug, Clone, Copy)]
pub enum Point {
    GoldenJoker(GoldenJoker),
    SilverJoker(SilverJoker),
    BigTwo(BigTwo),
    Ace(Ace),
    King(King),
    Queen(Queen),
    Jack(Jack),
    Ten(Ten),
    Nine(Nine),
    Eight(Eight),
    Seven(Seven),
    Six(Six),
    Five(Five),
    Four(Four),
    Three(Three),
    None,
}

impl Point {
    pub fn new(p: Point) -> Self {
        match p {
            Point::GoldenJoker(_) => Point::GoldenJoker(100),
            Point::SilverJoker(_) => Point::SilverJoker(90),
            Point::BigTwo(_) => Point::BigTwo(20),
            Point::Ace(_) => Point::Ace(14),
            Point::King(_) => Point::King(13),
            Point::Queen(_) => Point::Queen(12),
            Point::Jack(_) => Point::Jack(11),
            Point::Ten(_) => Point::Ten(10),
            Point::Nine(_) => Point::Nine(9),
            Point::Eight(_) => Point::Eight(8),
            Point::Seven(_) => Point::Seven(7),
            Point::Six(_) => Point::Six(6),
            Point::Five(_) => Point::Five(5),
            Point::Four(_) => Point::Four(4),
            Point::Three(_) => Point::Three(3),
            Point::None => Point::None,
        }
    }

    pub fn unwrap_point(&self) -> u8 {
        match *self {
            Point::GoldenJoker(x) => x,
            Point::SilverJoker(x) => x,
            Point::BigTwo(x) => x,
            Point::Ace(x) => x,
            Point::King(x) => x,
            Point::Queen(x) => x,
            Point::Jack(x) => x,
            Point::Ten(x) => x,
            Point::Nine(x) => x,
            Point::Eight(x) => x,
            Point::Seven(x) => x,
            Point::Six(x) => x,
            Point::Five(x) => x,
            Point::Four(x) => x,
            Point::Three(x) => x,
            Point::None => 0,
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.unwrap_point() == other.unwrap_point()
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Point {
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
        self.unwrap_point() > other.unwrap_point()
    }

    fn ge(&self, other: &Self) -> bool {
        self.unwrap_point() >= other.unwrap_point()
    }

    fn le(&self, other: &Self) -> bool {
        !self.gt(other)
    }

    fn lt(&self, other: &Self) -> bool {
        !self.ge(other)
    }
}
