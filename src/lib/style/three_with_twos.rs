use crate::lib::card::*;

#[derive(Debug)]
struct ThreeWithTwos(Box<[([Card; 3], [Card; 2])]>);
