use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

impl Card {
    pub fn value(&self) -> u32 {
        match self {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Jack => 11,
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 14,
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '2' => Some(Self::Two),
            '3' => Some(Self::Three),
            '4' => Some(Self::Four),
            '5' => Some(Self::Five),
            '6' => Some(Self::Six),
            '7' => Some(Self::Seven),
            '8' => Some(Self::Eight),
            '9' => Some(Self::Nine),
            'T' => Some(Self::Ten),
            'J' => Some(Self::Jack),
            'Q' => Some(Self::Queen),
            'K' => Some(Self::King),
            'A' => Some(Self::Ace),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandKind {
    None,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
}

impl Hand {
    pub fn from_cards(cards: &[Card; 5]) -> Self {
        Self { cards: *cards }
    }

    pub fn from_str(str: &str) -> Self {
        let mut cards: [Card; 5] = [Card::Two; 5];

        for (index, c) in str.chars().enumerate() {
            cards[index] = Card::from_char(c).unwrap();
        }

        Self { cards: cards }
    }

    pub fn kind(&self) -> HandKind {
        let mut cards = self.cards;
        cards.sort_unstable();

        if cards.windows(5).any(|w| w.iter().all(|c| c == &w[0])) {
            HandKind::FiveOfAKind
        } else if cards.windows(4).any(|w| w.iter().all(|c| c == &w[0])) {
            HandKind::FourOfAKind
        } else if {
            let mut first_match = &cards[0];

            cards.windows(3).any(|w| {
                first_match = &w[0];
                w.iter().all(|c| c == &w[0])
            }) && cards
                .windows(2)
                .any(|w| w.iter().all(|c| c != first_match && c == &w[0]))
        } {
            // Full house: (for any three cards: all are equal) AND (for any two cards: all are equal)
            HandKind::FullHouse
        } else if cards.windows(3).any(|w| w.iter().all(|c| c == &w[0])) {
            HandKind::ThreeOfAKind
        } else if {
            let mut first_match = &cards[0];

            cards.windows(2).any(|w| {
                first_match = &w[0];
                w.iter().all(|c| c == &w[0])
            }) && cards
                .windows(2)
                .any(|w| w.iter().all(|c| c != first_match && c == &w[0]))
        } {
            HandKind::TwoPair
        } else if cards.windows(2).any(|w| w.iter().all(|c| c == &w[0])) {
            HandKind::OnePair
        } else if cards.windows(2).all(|w| w[0] != w[1]) {
            HandKind::HighCard
        } else {
            HandKind::None
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (self_kind, other_kind) = (&self.kind(), &other.kind());

        match self_kind.cmp(other_kind) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Equal => {
                // Compare all cards.
                for index in 0..self.cards.len() {
                    let self_card = &self.cards[index];
                    let other_card = &other.cards[index];

                    match self_card.cmp(other_card) {
                        Ordering::Less => return Some(Ordering::Less),
                        Ordering::Greater => return Some(Ordering::Greater),
                        Ordering::Equal => {
                            continue;
                        }
                    }
                }

                Some(Ordering::Equal)
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_five_of_a_kind() {
        let hand = Hand::from_cards(&[Card::King; 5]);
        assert_eq!(HandKind::FiveOfAKind, hand.kind());
    }

    #[test]
    fn test_four_of_a_kind_front() {
        let hand = Hand::from_cards(&[Card::King, Card::King, Card::King, Card::King, Card::Queen]);
        assert_eq!(HandKind::FourOfAKind, hand.kind());
    }

    #[test]
    fn test_four_of_a_kind_back() {
        let hand = Hand::from_cards(&[Card::Ace, Card::Jack, Card::Jack, Card::Jack, Card::Jack]);
        assert_eq!(HandKind::FourOfAKind, hand.kind());
    }

    #[test]
    fn test_full_house_three_first() {
        let hand = Hand::from_cards(&[Card::Nine, Card::Nine, Card::Nine, Card::Two, Card::Two]);
        assert_eq!(HandKind::FullHouse, hand.kind());
    }

    #[test]
    fn test_full_house_two_first() {
        let hand = Hand::from_cards(&[Card::Six, Card::Six, Card::Eight, Card::Eight, Card::Eight]);
        assert_eq!(HandKind::FullHouse, hand.kind());
    }

    #[test]
    fn test_three_of_a_kind_front() {
        let hand = Hand::from_cards(&[Card::Four, Card::Four, Card::Four, Card::Six, Card::Eight]);
        assert_eq!(HandKind::ThreeOfAKind, hand.kind());
    }

    #[test]
    fn test_three_of_a_kind_center() {
        let hand = Hand::from_cards(&[Card::Two, Card::Jack, Card::Jack, Card::Jack, Card::Eight]);
        assert_eq!(HandKind::ThreeOfAKind, hand.kind());
    }

    #[test]
    fn test_three_of_a_kind_back() {
        let hand = Hand::from_cards(&[Card::Six, Card::Ace, Card::Two, Card::Two, Card::Two]);
        assert_eq!(HandKind::ThreeOfAKind, hand.kind());
    }

    #[test]
    fn test_three_of_a_kind_mixed() {
        let hand = Hand::from_cards(&[Card::Six, Card::Seven, Card::Six, Card::Ace, Card::Six]);
        assert_eq!(HandKind::ThreeOfAKind, hand.kind());
    }

    #[test]
    fn test_two_pair_back_to_back() {
        let hand = Hand::from_cards(&[Card::Six, Card::Six, Card::Ace, Card::Ace, Card::Two]);
        assert_eq!(HandKind::TwoPair, hand.kind());
    }

    #[test]
    fn test_two_pair_mixed() {
        let hand = Hand::from_cards(&[Card::Two, Card::Two, Card::Six, Card::King, Card::King]);
        assert_eq!(HandKind::TwoPair, hand.kind());
    }

    #[test]
    fn test_one_pair() {
        let hand = Hand::from_cards(&[Card::Two, Card::Two, Card::Eight, Card::Six, Card::Ace]);
        assert_eq!(HandKind::OnePair, hand.kind());
    }

    #[test]
    fn test_high_card_run() {
        let hand = Hand::from_cards(&[Card::Ace, Card::Queen, Card::King, Card::Jack, Card::Ten]);
        assert_eq!(HandKind::HighCard, hand.kind());
    }

    #[test]
    fn test_high_card_mixed() {
        let hand = Hand::from_cards(&[Card::Ace, Card::Two, Card::King, Card::Three, Card::Queen]);
        assert_eq!(HandKind::HighCard, hand.kind());
    }

    #[test]
    fn test_five_of_a_kind_vs_high_card() {
        let hand_1 = Hand::from_cards(&[Card::Ace; 5]);
        let hand_2 = Hand::from_cards(&[Card::Ace, Card::Queen, Card::Six, Card::Two, Card::Nine]);

        assert_eq!(&hand_1, hand_1.compare_hand(&hand_2).unwrap());
    }

    #[test]
    fn test_two_five_of_a_kind() {
        let hand_1 = Hand::from_cards(&[Card::Ace; 5]);
        let hand_2 = Hand::from_cards(&[Card::Six; 5]);

        assert_eq!(&hand_1, hand_1.compare_hand(&hand_2).unwrap());
    }
}
