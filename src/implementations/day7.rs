use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use strum::{EnumCount, EnumIter, EnumString};

use crate::utility::generic_error::{GenericError, GenericResult};

static JOKERS_ENABLED: AtomicBool = AtomicBool::new(false);

#[derive(EnumCount, EnumIter, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, EnumString)]
enum Card {
    #[strum(serialize = "X")] // Note: this is never present, Jacks are turned into Jokers when joker functionality is enabled
    Joker,
    #[strum(serialize = "2")]
    Two,
    #[strum(serialize = "3")]
    Three,
    #[strum(serialize = "4")]
    Four,
    #[strum(serialize = "5")]
    Five,
    #[strum(serialize = "6")]
    Six,
    #[strum(serialize = "7")]
    Seven,
    #[strum(serialize = "8")]
    Eight,
    #[strum(serialize = "9")]
    Nine,
    #[strum(serialize = "T")]
    Ten,
    #[strum(serialize = "J")]
    Jack,
    #[strum(serialize = "Q")]
    Queen,
    #[strum(serialize = "K")]
    King,
    #[strum(serialize = "A")]
    Ace,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
}

impl FromStr for Hand {
    type Err = GenericError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let jokers_enabled = JOKERS_ENABLED.load(Ordering::Relaxed);

        let mut cards: [Card; 5] = [Card::Two; 5];

        let mut tmp: [u8; 4] = [0; 4];
        for (index, char) in input.chars().enumerate() {
            let new_card = Card::from_str(char.encode_utf8(&mut tmp))?;
            cards[index] = if new_card == Card::Jack && jokers_enabled {
                Card::Joker
            } else {
                new_card
            }
        }

        let mut card_counts: Vec<(Card, u32)> = vec![];

        for card in cards {
            let found = card_counts.iter_mut().find(|(test, _)| *test == card);
            match found {
                Some((_, count)) => *count = *count + 1,
                None => card_counts.push((card, 1)),
            }
        }

        card_counts.sort_by(|(_, left), (_, right)| right.cmp(left));

        if jokers_enabled {
            if let Some(found) = card_counts
                .iter()
                .position(|(test, _)| *test == Card::Joker)
            {
                let joker_count = card_counts[found].1;
                if joker_count < 5 {
                    card_counts[if found == 0 { 1 } else { 0 }].1 += joker_count;
                    card_counts.remove(found);
                }
            }
        }

        let hand_type = match card_counts[..] {
            [(_, 5)] => HandType::FiveOfAKind,
            [(_, 4), (_, 1)] => HandType::FourOfAKind,
            [(_, 3), (_, 2)] => HandType::FullHouse,
            [(_, 3), (_, 1), (_, 1)] => HandType::ThreeOfAKind,
            [(_, 2), (_, 2), (_, 1)] => HandType::TwoPair,
            [(_, 2), (_, 1), (_, 1), (_, 1)] => HandType::OnePair,
            [(_, 1), (_, 1), (_, 1), (_, 1), (_, 1)] => HandType::HighCard,
            _ => {
                assert!(false);
                HandType::HighCard
            }
        };

        Ok(Hand {
            cards: cards,
            hand_type: hand_type,
        })
    }
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<std::cmp::Ordering> {
        if self.hand_type != other.hand_type {
            return self.hand_type.partial_cmp(&other.hand_type);
        }

        let cards = self.cards.iter().zip(other.cards.iter());
        for card in cards {
            if card.0 != card.1 {
                return card.0.partial_cmp(card.1);
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}

impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Hand) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn run_internal(input_path: &str) -> GenericResult<usize> {
    let file_contents = std::fs::read_to_string(input_path)?;

    let mut hands: Vec<(Hand, usize)> = vec![];
    for hand_str in file_contents.lines() {
        let parsed = sscanf::sscanf!(hand_str, "{} {}", str, usize)?;
        hands.push((Hand::from_str(parsed.0)?, parsed.1));
    }

    hands.sort_by(|(left, _), (right, _)| left.cmp(right));

    let mut result: usize = 0;

    for (index, (_, bet)) in hands.iter().enumerate() {
        result += (index + 1) * bet;
    }

    Ok(result)
}

#[test]
pub fn run_test_1() -> GenericResult<()> {
    JOKERS_ENABLED.store(false, Ordering::Relaxed);
    assert_eq!(run_internal("test_data/day7/example.txt")?, 6440);
    Ok(())
}

#[test]
pub fn run_test_2() -> GenericResult<()> {
    JOKERS_ENABLED.store(true, Ordering::Relaxed);
    assert_eq!(run_internal("test_data/day7/example.txt")?, 5905);
    Ok(())
}

pub fn run(input_path: &String) -> GenericResult<()> {
    JOKERS_ENABLED.store(false, Ordering::Relaxed);
    println!("Part one result: {}", run_internal(&input_path)?);
    JOKERS_ENABLED.store(true, Ordering::Relaxed);
    println!("Part two result: {}", run_internal(&input_path)?);
    Ok(())
}
