use std::cmp::Ordering;
use std::cmp::{PartialEq, PartialOrd};

use crate::lib::color::*;
use crate::lib::point::*;

#[derive(Debug, Clone, Copy)]
pub struct Card {
    point: Point,
    color: Color,
}

impl Card {
    pub fn new(p: Point, c: Color) -> Card {
        let tp = match p {
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
        };

        Card {
            point: tp,
            color: c,
        }
    }

    pub fn default() -> Card {
        Card {
            point: Point::None,
            color: Color::None,
        }
    }

    pub fn unwrap_point(&self) -> u8 {
        match self.point {
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

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.unwrap_point() == other.unwrap_point()
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Card {
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
