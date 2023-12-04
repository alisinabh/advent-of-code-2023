// Advent of code: Day 4
// Author: @alisinabh

use std::env;
use std::error::Error;
use std::fs::read_to_string;
use std::num::ParseIntError;

#[derive(Debug)]
struct ScratchCard {
    card_number: usize,
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>,
}

impl ScratchCard {
    fn parse(line: &str) -> Result<ScratchCard, Box<dyn Error>> {
        let [card, numbers] = line.split(':').collect::<Vec<_>>()[..] else { return Err("Bad input".into())};

        let [_, number] = card.split_whitespace().collect::<Vec<_>>()[..] else {return Err("Bad input".into())};

        let [winning_numbers, numbers] = numbers.split('|').collect::<Vec<_>>()[..] else {return Err("Bad input".into())};

        let mut winning_numbers = Self::parse_numbers(winning_numbers)?;
        let mut numbers = Self::parse_numbers(numbers)?;

        winning_numbers.sort();
        numbers.sort();

        Ok(ScratchCard {
            card_number: number.parse()?,
            winning_numbers: winning_numbers,
            numbers: numbers,
        })
    }

    fn calculate_power(&self) -> Option<i32> {
        match self.match_count() {
            0 => Some(0),
            c => 2_i32.checked_pow(c as u32 - 1),
        }
    }

    fn prize_card_count(&self, cards: &[Self]) -> usize {
        (0..self.match_count()).fold(1, |acc, i| acc + cards[i].prize_card_count(&cards[i + 1..]))
    }

    fn match_count(&self) -> usize {
        let mut matches = 0;
        let mut winner_cursor = 0;

        for &n in self.numbers.iter() {
            while winner_cursor < self.winning_numbers.len() {
                let winner_number = self.winning_numbers[winner_cursor];
                if n == winner_number {
                    matches += 1;
                    break;
                } else if n < winner_number {
                    break;
                } else {
                    winner_cursor += 1;
                }
            }
        }

        matches
    }

    fn parse_numbers(numbers: &str) -> Result<Vec<i32>, ParseIntError> {
        numbers.split_whitespace().map(|n| n.parse()).collect()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let file = read_to_string(&args[1])?;

    let mut power = 0;
    let mut cards: Vec<ScratchCard> = Vec::new();

    for line in file.lines() {
        let card = ScratchCard::parse(line)?;
        power = power + card.calculate_power().ok_or("overflow")?;
        cards.push(card);
    }

    println!("pile power {}", power);

    let prize_count =
        (0..cards.len()).fold(0, |acc, i| acc + cards[i].prize_card_count(&cards[i + 1..]));

    println!("pile prize count: {}", prize_count);

    Ok(())
}
