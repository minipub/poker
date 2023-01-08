pub mod boom;
pub mod chain;
pub mod iface;
pub mod pairs;
pub mod three_with_ones;
pub mod three_with_pairs;
pub mod threes;

#[derive(Debug)]
pub enum CardStyle {
    Boom(boom::Bomb),
    Chain(chain::Chain),
    Pairs(pairs::Pairs),
    ThreeWithOnes(three_with_ones::ThreeWithOnes),
    ThreeWithPairs(three_with_pairs::ThreeWithPairs),
    Threes(threes::Threes),
}
