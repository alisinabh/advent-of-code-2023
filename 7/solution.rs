// Advent of code: Day 7
// Author: @alisinabh

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt;

#[derive(Copy, Clone, Debug)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    r#type: HandType,
    bid: u64,
}

impl Hand {
    fn from_line(line: &str) -> Result<Self, Box<dyn Error>> {
        let [hand, bid] = line.split_whitespace().collect::<Vec<_>>()[..] else { return Err("invalid hand line".into()) };

        let bid: u64 = bid.parse()?;
        let cards: Vec<_> = hand
            .chars()
            .map(|c| Card::from_char(&c).ok_or("invalid char"))
            .collect::<Result<_, _>>()?;

        let hand_type = Self::calculate_type(&cards);

        Ok(Hand {
            cards: cards,
            bid: bid,
            r#type: hand_type,
        })
    }

    fn calculate_type(cards: &Vec<Card>) -> HandType {
        let mut map: HashMap<u8, u8> = HashMap::new();

        for c in cards {
            map.entry(c.power)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        match map.values().max().unwrap() {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if map.keys().len() == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if map.values().filter(|&&x| x == 2).count() == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            _ => HandType::HighCard,
        }
    }

    fn magic_number(&self) -> u64 {
        self.cards.iter().enumerate().fold(
            (self.r#type as u64) * 10_u64.pow(10),
            |acc, (indx, value)| acc + 10_u64.pow((8 - indx * 2) as u32) * value.power as u64,
        )
    }
}

struct Card {
    power: u8,
}

const CARD_ORDER: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

impl Card {
    fn from_char(c: &char) -> Option<Self> {
        if let Some(indx) = CARD_ORDER.iter().position(|&r| r == *c) {
            Some(Card {
                power: (indx + 1) as u8,
            })
        } else {
            None
        }
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Card")
            .field("power", &CARD_ORDER[(self.power - 1) as usize])
            .finish()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let data = std::fs::read_to_string(&args[1])?;

    let mut hands = Vec::new();

    for line in data.lines() {
        hands.push(Hand::from_line(line)?);
    }

    hands.sort_by(|a, b| a.magic_number().cmp(&b.magic_number()));

    let mut rank = 1;
    let mut sum = 0;

    for h in hands {
        sum += rank * h.bid;
        rank += 1;
    }

    println!("part one: {}", sum);

    Ok(())
}
