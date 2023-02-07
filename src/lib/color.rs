#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum Color {
    Spades,
    Plum,
    Square,
    Hearts,
    None,
}

impl Default for Color {
    fn default() -> Self {
        Self::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let spades_0 = Color::Spades;
        let spades_1 = Color::Spades;
        let plum = Color::Plum;
        let square = Color::Square;
        let hearts = Color::Hearts;

        assert_eq!(true, spades_0 == spades_1);
        assert_eq!(false, spades_0 == plum);

        assert_eq!(true, spades_0 < plum);
        assert_eq!(true, spades_0 < square);
        assert_eq!(true, spades_0 < hearts);
        assert_eq!(true, plum < square);
        assert_eq!(true, plum < hearts);
        assert_eq!(true, square < hearts);
    }
}
