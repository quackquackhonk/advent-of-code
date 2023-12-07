use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub hand_type: HandType,
    pub bid: usize,
}

impl Hand {
    pub fn new(cards: Vec<Card>, bid: usize) -> Self {
        let hand_type = HandType::find_hand_type(&cards);

        Self {
            cards,
            hand_type,
            bid,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => {
                for (s, o) in self.cards.iter().zip(other.cards.iter()) {
                    match s.cmp(o) {
                        std::cmp::Ordering::Equal => continue,
                        ord => return ord,
                    }
                }

                std::cmp::Ordering::Equal
            }
            ord => ord,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl HandType {
    pub fn find_hand_type(cards: &[Card]) -> Self {
        let counts = HandType::count_cards(cards);
        match counts.values().len() {
            1 => Self::FiveKind,
            2 => {
                // Either FourKind or FullHouse
                match counts.values().max() {
                    Some(4) => Self::FourKind,
                    Some(_) => Self::FullHouse,
                    None => unreachable!("We should always have a max value"),
                }
            }
            3 => {
                // Either ThreeKind or TwoPair
                match counts.values().max() {
                    Some(3) => Self::ThreeKind,
                    Some(_) => Self::TwoPair,
                    None => unreachable!("We should always have a max value"),
                }
            }
            4 => Self::OnePair,
            5 => Self::HighCard,
            x => unreachable!("different cards {}", x),
        }
    }

    pub fn count_cards(cards: &[Card]) -> HashMap<Card, usize> {
        let mut map = HashMap::new();
        for card in cards {
            match map.get(card) {
                Some(x) => map.insert(*card, x + 1),
                None => map.insert(*card, 1),
            };
        }

        map
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Ace,
            "K" => Self::King,
            "Q" => Self::Queen,
            "J" => Self::Jack,
            "T" => Self::Ten,
            "9" => Self::Nine,
            "8" => Self::Eight,
            "7" => Self::Seven,
            "6" => Self::Six,
            "5" => Self::Five,
            "4" => Self::Four,
            "3" => Self::Three,
            "2" => Self::Two,
            _ => unreachable!("Got {}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::parse::parse_cards;

    use super::*;

    #[rstest]
    #[case("32T3K", HandType::OnePair)]
    #[case("T55J5", HandType::ThreeKind)]
    #[case("KK677", HandType::TwoPair)]
    #[case("KTJJT", HandType::TwoPair)]
    #[case("QQQJA", HandType::ThreeKind)]
    #[case("K2345", HandType::HighCard)]
    #[case("AAA44", HandType::FullHouse)]
    #[case("AA5AA", HandType::FourKind)]
    #[case("44444", HandType::FiveKind)]
    pub fn test_find_hand_type(#[case] input: &str, #[case] expected: HandType) {
        let (_, cards) = parse_cards(input).expect("THIS SHOULD PARSE");
        assert_eq!(HandType::find_hand_type(&cards), expected)
    }
}
