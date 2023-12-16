use anyhow::Result;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::slice::Iter;

fn main() {
    let input = include_str!("example.in");
    let p1 = part1(input);
    dbg!(p1.unwrap());
    let p2 = part2(input);
    dbg!(p2.unwrap());
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Card {
    pub fn iterator() -> Iter<'static, Card> {
        static CARDS: [Card; 13] = [
            Card::Two,
            Card::Three,
            Card::Four,
            Card::Five,
            Card::Six,
            Card::Seven,
            Card::Eight,
            Card::Nine,
            Card::Ten,
            Card::Jack,
            Card::Queen,
            Card::King,
            Card::Ace,
        ];
        CARDS.iter()
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    hand_type: HandType,
}

impl Hand {
    fn new(cards: String, bid: u32, jokers: bool) -> Hand {
        let cards: Vec<Card> = cards
            .chars()
            .map(|x| match x {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'J' => {
                    if jokers {
                        Card::Joker
                    } else {
                        Card::Jack
                    }
                }
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => panic!("Invalid card"),
            })
            .collect();
        let mut card_counts = Vec::new();
        let mut joker_count = 0;
        Card::iterator().for_each(|card_type| {
            if card_type == &Card::Joker {
                joker_count += 1;
            } else {
                let count = cards
                    .iter()
                    .filter(|in_hand| {
                        return &card_type == in_hand;
                    })
                    .count();
                if count > 0 {
                    card_counts.push(count);
                }
            }
        });
        if joker_count > 0 {
            let (max_index, _) = card_counts
                .iter()
                .enumerate()
                .max_by_key(|(_, &value)| value)
                .unwrap();
            card_counts[max_index] += joker_count;
        }
        let hand_type = match card_counts.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if card_counts.contains(&4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if card_counts.contains(&3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        };
        Hand {
            cards,
            bid,
            hand_type,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        } else {
            for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                if a != b {
                    return a.cmp(b);
                }
            }
            return Ordering::Equal;
        }
    }
}
fn part1(input: &str) -> Result<usize> {
    let mut hands: Vec<Hand> = Vec::new();
    for (cards, bid) in input.lines().map(|x| x.split_once(" ").unwrap()) {
        hands.push(Hand::new(
            cards.to_string(),
            bid.parse::<u32>().unwrap(),
            false,
        ));
    }
    hands.sort();
    let mut total = 0;
    for (index, hand) in hands.iter().enumerate() {
        total += (index + 1) as usize * hand.bid as usize;
    }
    Ok(total)
}

fn part2(input: &str) -> Result<usize> {
    let mut hands: Vec<Hand> = Vec::new();
    for (cards, bid) in input.lines().map(|x| x.split_once(" ").unwrap()) {
        hands.push(Hand::new(
            cards.to_string(),
            bid.parse::<u32>().unwrap(),
            true,
        ));
    }
    hands.sort();
    let mut total = 0;
    for (index, hand) in hands.iter().enumerate() {
        total += (index + 1) as usize * hand.bid as usize;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        let expected = 6440;
        let result = part1(include_str!("example.in"));
        println!("expected: {:?}, received {:?}", expected, result);
        assert_eq!(result.is_ok_and(|x| x == expected), true);
    }
}
