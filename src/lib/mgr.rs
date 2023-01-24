use crate::lib::card::*;
use crate::lib::color::*;
use crate::lib::deck::*;
use crate::lib::judger::*;
use crate::lib::player::*;
use crate::lib::point::*;
use crate::lib::session::*;
use crate::lib::style::chain::Chain;
use crate::lib::style::iface::Suit;
use crate::lib::style::pairs::Pairs;
use crate::lib::style::three_with_ones::ThreeWithOnes;

pub fn test() {
    // let t1 = Card::new(Point::Ten(0), Color::Spades);

    // let t2 = Card::new(Point::King(0), Color::Plum);

    // let t3 = Card::new(Point::Jack(0), Color::Square);

    // let t4 = Card::new(Point::Queen(0), Color::Square);

    // let t5 = Card::new(Point::Ace(0), Color::Hearts);

    // println!("{:?}", t1 < t2);

    // let mut pc = Chain(Box::new(vec![]));
    // let ce = pc.suit(Box::new(vec![t1, t2, t3, t4, t5]));
    // println!("Chain: {:?}", pc);
    // println!("ce: {:?}", ce);

    // let mut pp = Pairs(Box::new(vec![]));
    // let pe = pp.suit(Box::new(vec![t3, t2, t4, t3, t4, t2]));
    // println!("Pair: {:?}", pp);
    // println!("pe: {:?}", pe);

    // let vs = Deck::new(1);
    // // println!("deck card: {:?}", vs);

    // let mut j = Judger::new(vs);
    // j.shuffle();

    // println!("judger shuffle: {:?}", j);
    // println!("card num: {:?}", j.cards_count());

    // j.reserve(3);
    // println!("judger reserve: {:?}", j);

    // let mut p1 = Player::new(1, "john");
    // let mut p2 = Player::new(2, "mike");
    // let mut p3 = Player::new(3, "alex");

    // j.deal([&mut p1, &mut p2, &mut p3]);

    // j.deal_lord(&mut p2);

    // println!("p1: {:?}, {}", p1, p1.cards_count());
    // println!("p2: {:?}, {}", p2, p2.cards_count());
    // println!("p3: {:?}, {}", p3, p3.cards_count());

    // let mut pto = ThreeWithOnes(Box::new(vec![]));
    // let ptoe = pto.suit(&vec![t1, t1, t2, t1, t3, t5, t3, t3]);
    // println!("ThreeWithOnes: {:?}", pto);
    // println!("ptoe: {:?}", ptoe);
}

pub fn manage() {
    // let mut s = Session::new(1001, SessionType::Three);

    // let mut p1 = Player::new(100, "john");
    // let mut p2 = Player::new(101, "mike");
    // let mut p3 = Player::new(102, "alex");

    // s.push_player(p1);
    // s.push_player(p2);
    // s.push_player(p3);

    // let started = s.begin();
    // if started {
    //     s.set_lord(2);
    // }

    // println!("session: {:?}", s);

    // // s.play_round(p3);
}
