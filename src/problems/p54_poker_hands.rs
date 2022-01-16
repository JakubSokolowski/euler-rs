use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

const VALUES: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A",
];

#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    suit: String,
    value: String,
}

impl Card {
    fn int_value(&self) -> usize {
        get_int_value(&self.value)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

fn get_int_value(value: &str) -> usize {
    VALUES.iter().position(|v| *v == value).unwrap()
}

impl Eq for Card {}

impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.int_value().cmp(&other.int_value()))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.int_value().cmp(&other.int_value())
    }
}

#[derive(Debug, PartialEq, Clone, Ord, PartialOrd, Eq, Copy)]
pub enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPairs = 2,
    ThreeOfAKind = 3,
    Straight = 4,
    Flush = 5,
    FullHouse = 6,
    FourOfAKind = 7,
    StraightFlush = 8,
    RoyalFlush = 9,
}

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let line = self
            .cards
            .iter()
            .map(|c| format!("{}{}", c.value, c.suit))
            .join(" ");
        write!(f, "[{}]", line)
    }
}

pub fn player_1_wins(player1: &Hand, player2: &Hand) -> bool {
    let first = player1.best_hand();
    let second = player2.best_hand();

    if first == second {
        let better = same_hand_better(player1, player2, first);
        println!(
            "Same hand: {:?}{} {:?}{} First better: {}",
            first, player1, second, player2, better
        );
        better
    } else {
        first > second
    }
}

pub fn same_hand_better(player1: &Hand, player2: &Hand, h_type: HandType) -> bool {
    if h_type == HandType::HighCard {
        return highest_card_better(player1, player2);
    }
    let first_hand_highest = player1.most_count_value();
    let second_hand_highest = player2.most_count_value();

    if first_hand_highest == second_hand_highest {
        println!(
            "\tSame Highest: {:?} {:?}",
            first_hand_highest, second_hand_highest
        );
        let highest_1 = player1.highest_card();
        let highest_2 = player2.highest_card();
        highest_1 > highest_2
    } else {
        first_hand_highest > second_hand_highest
    }
}

pub fn highest_card_better(player1: &Hand, player2: &Hand) -> bool {
    for (a, b) in player1.cards.iter().rev().zip(player2.cards.iter().rev()) {
        if a.int_value() == b.int_value() {
            continue;
        }
        return a.int_value() > b.int_value();
    }
    false
}

impl Hand {
    pub fn new(cards: &[Card]) -> Hand {
        let sorted_cards: Vec<_> = cards.iter().cloned().sorted().collect();
        Hand {
            cards: sorted_cards,
        }
    }

    pub fn highest_card(&self) -> &Card {
        self.cards.iter().last().unwrap()
    }

    pub fn most_count_value(&self) -> usize {
        let counts = self.cards.iter().counts_by(|c| c.value.clone());

        let max_value = counts.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0;

        get_int_value(max_value)
    }

    fn best_hand(&self) -> HandType {
        if self.is_royal_flush() {
            return HandType::RoyalFlush;
        }

        if self.is_straight_flush() {
            return HandType::StraightFlush;
        }

        if self.is_four_of_a_kind() {
            return HandType::FourOfAKind;
        }

        if self.is_full_house() {
            return HandType::FullHouse;
        }

        if self.is_flush() {
            return HandType::Flush;
        }

        if self.is_straight() {
            return HandType::Straight;
        }

        if self.is_three_of_a_kind() {
            return HandType::ThreeOfAKind;
        }

        if self.is_two_pairs() {
            return HandType::TwoPairs;
        }

        if self.is_one_pair() {
            return HandType::OnePair;
        }

        HandType::HighCard
    }

    fn is_three_of_a_kind(&self) -> bool {
        self.is_multiple(3)
    }

    fn is_one_pair(&self) -> bool {
        self.is_multiple(2)
    }

    fn is_two_pairs(&self) -> bool {
        let counts = self.cards.iter().counts_by(|c| c.value.clone());

        counts.values().filter(|v| **v == 2).count() == 2
    }

    fn is_multiple(&self, value: usize) -> bool {
        let counts = self.cards.iter().counts_by(|c| c.value.clone());

        counts.values().contains(&value)
    }

    fn is_full_house(&self) -> bool {
        self.is_three_of_a_kind() & self.is_one_pair()
    }

    fn is_flush(&self) -> bool {
        self.cards.iter().all(|c| c.suit == self.cards[0].suit)
    }

    fn is_straight(&self) -> bool {
        self.cards
            .windows(2)
            .all(|w| w[1].int_value() - w[0].int_value() == 1)
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.is_multiple(4)
    }

    fn is_straight_flush(&self) -> bool {
        self.is_flush() && self.is_straight()
    }

    fn is_royal_flush(&self) -> bool {
        self.is_straight_flush() && self.cards[0].value == "10"
    }
}

pub fn parse_cards(cards_str: &str) -> Hand {
    let cards: Vec<_> = cards_str
        .trim()
        .split(' ')
        .map(|c| Card {
            value: c.chars().next().unwrap().to_string(),
            suit: c.chars().nth(1).unwrap().to_string(),
        })
        .collect();

    Hand::new(&cards)
}

fn parse_line(line: &str) -> (Hand, Hand) {
    let first = &line[..15];
    let second = &line[15..];

    (parse_cards(first), parse_cards(second))
}

pub fn count_winners(lines: &[String]) -> usize {
    lines
        .iter()
        .filter_map(|l| {
            let (first, second) = parse_line(l);
            if player_1_wins(&first, &second) {
                Some(1)
            } else {
                None
            }
        })
        .count()
}

pub fn run(lines: &[String]) {
    let count = count_winners(lines);
    println!("Player 1 won: {} times", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_best_hand_full_house() {
        // given
        let hand = parse_cards("2H 2D 4C 4D 4S");

        // when
        let result = hand.best_hand();

        // then
        let expected = HandType::FullHouse;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_best_hand_three_of_a_kind() {
        // given
        let hand = parse_cards("2D 9C AS AH AC");

        // when
        let result = hand.best_hand();

        // then
        let expected = HandType::ThreeOfAKind;
        assert_eq!(expected, result);
    }
}
