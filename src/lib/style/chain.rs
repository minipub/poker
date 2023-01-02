use crate::lib::card::*;
use crate::lib::style::iface::Suit;

#[derive(Debug)]
pub struct Chain(pub Box<Vec<Card>>);

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
