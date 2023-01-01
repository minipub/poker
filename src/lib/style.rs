use crate::lib::card::*;

#[derive(Debug)]
enum CardStyle {
    Threes(Threes),
}

#[derive(Debug)]
struct Bomb(Box<[Card; 4]>);

#[derive(Debug)]
struct Threes(Box<[[Card; 3]]>);

#[derive(Debug)]
struct ThreeWithOnes(Box<[([Card; 3], [Card; 1])]>);

#[derive(Debug)]
struct ThreeWithTwos(Box<[([Card; 3], [Card; 2])]>);

#[derive(Debug)]
pub struct Chain(pub Box<Vec<Card>>);

#[derive(Debug)]
pub struct Pairs(pub Box<Vec<[Card; 2]>>);

pub trait Suit {
    type Error;
    fn suit(&mut self, _: Box<Vec<Card>>) -> Option<Self::Error>;
}

impl Suit for Chain {
    type Error = &'static str;

    fn suit(&mut self, cs: Box<Vec<Card>>) -> Option<Self::Error> {
        if cs.len() < 5 {
            return Some("not reach 5 elements.");
        }

        let mut v = vec![Card::default(); cs.len()];
        v.clone_from_slice(&cs);

        // println!("before sort, v: {:?}", v);
        v.sort_by(|x, y| x.partial_cmp(y).unwrap());
        // println!("after sort, v: {:?}", v);

        let mut m = v[0].unwrap_point();
        for x in &v {
            let xp = x.unwrap_point();
            if m != xp {
                return Some("not continous.");
            }
            m = xp + 1;
        }

        self.0 = Box::new(v);
        None
    }
}

impl Suit for Pairs {
    type Error = &'static str;

    fn suit(&mut self, cs: Box<Vec<Card>>) -> Option<Self::Error> {
        if cs.len() < 2 {
            return Some("chain pairs number must be ge 2.");
        }
        if cs.len() == 4 {
            return Some("chain pairs number can't be 4.");
        }
        if cs.len() % 2 == 1 {
            return Some("chain pairs number must be plural.");
        }

        let mut v = vec![Card::default(); cs.len()];
        v.clone_from_slice(&cs);

        // println!("before sort, v: {:?}", v);
        v.sort_by(|x, y| x.partial_cmp(y).unwrap());
        // println!("after sort, v: {:?}", v);

        let mut v2 = vec![[Card::default(); 2]; cs.len() / 2];
        let mut m1 = v[0].unwrap_point();
        let mut m2 = v[1].unwrap_point();
        let mut i = 0;

        while i < cs.len() {
            let xp = v[i].unwrap_point();
            let yp = v[i + 1].unwrap_point();

            if xp != yp || m1 != m2 {
                return Some("not a pair.");
            }

            if m1 != xp {
                return Some("not continous.");
            }

            v2[i / 2] = [v[i], v[i + 1]];

            m1 = xp + 1;
            m2 = m1;
            i += 2;
        }

        self.0 = Box::new(v2);

        None
    }
}
