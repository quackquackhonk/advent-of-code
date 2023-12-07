#[derive(Debug, PartialEq)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub hand_type: HandType,
    pub bid: usize,
}

impl Hand {
    pub fn new(cards: Vec<Card>, bid: usize) -> Self {
        let hand_type = HandType::find_hand_type(&cards);

        Self {
            cards, hand_type, bid
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
    FiveKind
}

impl HandType {
    pub fn find_hand_type(cards: &[Card]) -> Self {
        dbg!("Find hand type of {}", cards);
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
    Ace
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Ace,
            "K" => Self::King,
            "Q" => Self::Queen,
            "J" => Self::Jack,
            "9" => Self::Nine,
            "8" => Self::Eight,
            "7" => Self::Seven,
            "6" => Self::Six,
            "5" => Self::Five,
            "4" => Self::Four,
            "3" => Self::Three,
            "2" => Self::Two,
        }
    }
}
