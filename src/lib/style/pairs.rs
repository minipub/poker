use crate::lib::card::*;
use crate::lib::style::interface::Suit;

#[derive(Debug)]
pub struct Pairs(pub Box<Vec<[Card; 2]>>);

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
