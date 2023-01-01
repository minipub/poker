use rand::prelude::*;
use std::cmp::Ordering;
use std::cmp::{PartialEq, PartialOrd};

/// Deck of Cards
#[derive(Debug)]
struct Deck {}

impl Deck {
    fn new(cnt: u8) -> Box<Vec<Card>> {
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

#[derive(Debug)]
struct Player<'a> {
    id: u8,
    name: &'a str,
    is_lord: bool,
    cards: Box<Vec<Card>>,
}

impl<'a> Player<'a> {
    fn new(id: u8, name: &str) -> Player {
        Player {
            id,
            name,
            is_lord: false,
            cards: Box::new(vec![]),
        }
    }
}

#[derive(Debug)]
struct Judger {
    cards: Box<Vec<Card>>,
    lords: Box<Vec<Card>>,
}

impl Judger {
    fn new(vs: Box<Vec<Card>>) -> Judger {
        Judger {
            cards: vs,
            lords: Box::new(vec![]),
        }
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn reserve(&mut self, cnt: u8) {
        let mut i = 0;

        while i < cnt {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0..self.cards.len());
            let t = self.cards.remove(x);
            self.lords.push(t);
            i += 1;
        }
    }

    fn deal_lord(&mut self, p: &mut Player) {
        let mut next = true;
        while next {
            next = match self.lords.pop() {
                Some(t) => {
                    p.cards.push(t);
                    true
                }
                None => false,
            }
        }
        p.is_lord = true;
    }

    // fn cut(&mut self) {
    //     let mut rng = rand::thread_rng();
    //     let x: u8 = rng.gen_range(0..(self.cards.len() as u8));

    //     let mcnt = self.cards.len() / 2;
    //     let mut i = mcnt;
    //     while i < self.cards.len() {
    //         let tmp = self.cards[i];
    //         self.cards[i] = self.cards[i - mcnt];
    //         self.cards[i - mcnt] = tmp;
    //         i += 1;
    //     }
    // }

    fn deal(&mut self, mut ps: [&mut Player; 3]) {
        let mut next = true;
        while next {
            for p in ps.iter_mut() {
                next = self.deal_one_card(p);
            }
        }
    }

    fn deal_one_card(&mut self, p: &mut Player) -> bool {
        match self.cards.pop() {
            Some(t) => {
                p.cards.push(t);
                true
            }
            None => false,
        }
    }
}

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
struct Chain(Box<Vec<Card>>);

#[derive(Debug)]
struct Pairs(Box<Vec<[Card; 2]>>);

trait Suit {
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

#[derive(Debug, Clone, Copy)]
struct Card {
    point: Point,
    color: Color,
}

impl Card {
    fn new(p: Point, c: Color) -> Card {
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

    fn default() -> Card {
        Card {
            point: Point::None,
            color: Color::None,
        }
    }

    fn unwrap_point(&self) -> u8 {
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

#[derive(Debug, Clone, Copy)]
enum Point {
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
enum Color {
    Spades,
    Plum,
    Square,
    Hearts,
    None,
}

fn main() {
    let t1 = Card::new(Point::Ten(0), Color::Spades);

    let t2 = Card::new(Point::King(0), Color::Plum);

    let t3 = Card::new(Point::Jack(0), Color::Square);

    let t4 = Card::new(Point::Queen(0), Color::Square);

    let t5 = Card::new(Point::Ace(0), Color::Hearts);

    println!("{:?}", t1 < t2);

    let mut pc = Chain(Box::new(vec![]));
    let ce = pc.suit(Box::new(vec![t1, t2, t3, t4, t5]));
    println!("Chain: {:?}", pc);
    println!("ce: {:?}", ce);

    let mut pp = Pairs(Box::new(vec![]));
    let pe = pp.suit(Box::new(vec![t3, t2, t4, t3, t4, t2]));
    println!("Pair: {:?}", pp);
    println!("pe: {:?}", pe);

    let vs = Deck::new(1);
    // println!("deck card: {:?}", vs);

    let mut j = Judger::new(vs);
    j.shuffle();

    println!("judger shuffle: {:?}", j.cards);
    println!("card num: {:?}", j.cards.len());

    j.reserve(3);
    println!("judger reserve: {:?}", j);

    let mut p1 = Player::new(1, "john");
    let mut p2 = Player::new(2, "mike");
    let mut p3 = Player::new(3, "alex");

    j.deal([&mut p1, &mut p2, &mut p3]);

    j.deal_lord(&mut p2);

    println!("p1: {:?}, {}", p1, p1.cards.len());
    println!("p2: {:?}, {}", p2, p2.cards.len());
    println!("p3: {:?}, {}", p3, p3.cards.len());
}
