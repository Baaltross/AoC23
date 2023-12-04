use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

use crate::utility;

struct Card {
    _id: u32,
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
    num_matches: usize,
}

impl Card {
    fn new(id: u32) -> Card {
        Card {
            _id: id,
            winning_numbers: vec![],
            your_numbers: vec![],
            num_matches: 0,
        }
    }
}

impl FromStr for Card {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split_str: Vec<&str> = input.split(&[':', '|']).collect();
        let card_id = sscanf::sscanf!(split_str[0], "Card{:/[\x20]+/}{}", str, usize).unwrap();

        let mut output: Card = Card::new(card_id.1.try_into().unwrap());

        for number_str in split_str[1].split_whitespace() {
            if number_str.len() == 0 {
                continue;
            }

            output.winning_numbers.push(number_str.parse()?);
        }

        for number_str in split_str[2].split_whitespace() {
            if number_str.len() == 0 {
                continue;
            }

            output.your_numbers.push(number_str.parse()?);
        }

        for winning_number in output.winning_numbers.iter() {
            for your_number in output.your_numbers.iter() {
                if winning_number == your_number {
                    output.num_matches += 1;
                }
            }
        }

        Ok(output)
    }
}

fn part_1(input_path: &str) -> utility::generic_error::GenericResult<u32> {
    let file_handle = std::fs::File::open(input_path)?;
    let reader = BufReader::new(file_handle);

    let mut result = 0;

    let mut cards: Vec<Card> = vec![];

    for line in reader.lines() {
        let unwrapped_line = line?;

        cards.push(Card::from_str(unwrapped_line.as_str())?)
    }

    for card in cards {
        if card.num_matches > 0 {
            result += 1 << (card.num_matches - 1);
        }
    }

    Ok(result)
}

fn part_2(input_path: &str) -> utility::generic_error::GenericResult<u32> {
    let file_handle = std::fs::File::open(input_path)?;
    let reader = BufReader::new(file_handle);

    let mut result = 0;

    let mut cards: Vec<Card> = vec![];

    for line in reader.lines() {
        let unwrapped_line = line?;

        cards.push(Card::from_str(unwrapped_line.as_str()).unwrap())
    }

    let mut number_of_each_card:Vec<u32> = vec![1;cards.len()];

    for (index, card) in cards.iter().enumerate() {
        let number_of_this_card = number_of_each_card[index];
        for i in 0..card.num_matches {
            if let Some(elem) = number_of_each_card.get_mut(index + i + 1) {
                *elem += number_of_this_card;
            }
        }

        result += number_of_this_card;
    }

    Ok(result)
}

#[test]
pub fn run_test_1() -> std::io::Result<()> {
    assert_eq!(part_1("test_data/day4/example.txt").unwrap(), 13);
    Ok(())
}

#[test]
pub fn run_test_2() -> std::io::Result<()> {
    assert_eq!(part_2("test_data/day4/example.txt").unwrap(), 30);
    Ok(())
}

pub fn run(input_path: &String) -> utility::generic_error::GenericResult<()> {
    println!("Part one result: {}", part_1(&input_path)?);
    println!("Part two result: {}", part_2(&input_path)?);
    Ok(())
}
