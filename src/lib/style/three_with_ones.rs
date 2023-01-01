use crate::lib::card::*;

#[derive(Debug)]
struct ThreeWithOnes(Box<[([Card; 3], [Card; 1])]>);
