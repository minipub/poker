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
