use std::cmp::Ordering;
use std::cmp::{PartialEq, PartialOrd};

use crate::lib::color::*;
use crate::lib::point::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Card {
    point: Point,
    color: Color,
}

impl Card {
    pub fn new(p: Point, c: Color) -> Card {
        Card {
            point: Point::new(p),
            color: c,
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

// compare Card = compare Point and Color both
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.point.unwrap_point() == other.point.unwrap_point() && self.color == other.color
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            Some(Ordering::Equal)
        } else if self.gt(other) {
            Some(Ordering::Greater)
        } else if self.lt(other) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }

    fn gt(&self, other: &Self) -> bool {
        let x = self.point.unwrap_point();
        let y = other.point.unwrap_point();
        if x > y {
            true
        } else if x == y {
            self.color > other.color
        } else {
            false
        }
    }

    fn lt(&self, other: &Self) -> bool {
        if self.point < other.point {
            true
        } else if self.point == other.point {
            self.color < other.color
        } else {
            false
        }
    }

    fn ge(&self, other: &Self) -> bool {
        !self.lt(other)
    }

    fn le(&self, other: &Self) -> bool {
        !self.gt(other)
    }
}
