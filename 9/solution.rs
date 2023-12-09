// Advent of code: Day 9
// Author: @alisinabh

use std::env;
use std::error::Error;

#[derive(Debug)]
struct Serie {
    readings: Vec<i64>,
}

impl Serie {
    fn parse(line: &str) -> Result<Self, Box<dyn Error>> {
        let readings = line
            .split_whitespace()
            .map(|r| r.parse::<i64>())
            .collect::<Result<_, _>>()?;

        Ok(Self { readings })
    }

    fn next_value(&self) -> i64 {
        self.extrapolate()
            .iter()
            .rev()
            .fold(0, |acc, x| acc + x.last().unwrap())
    }

    fn previous_value(&self) -> i64 {
        self.extrapolate()
            .iter()
            .rev()
            .fold(0, |acc, x| x.first().unwrap() - acc)
    }
    fn extrapolate(&self) -> Vec<Vec<i64>> {
        let mut extrapolated: Vec<Vec<i64>> = Vec::new();

        extrapolated.push(self.readings.clone());

        loop {
            let items = Self::single_extrapolate(&extrapolated.last().unwrap());

            if items.iter().all(|&x| x == 0) {
                break;
            } else {
                extrapolated.push(items);
            }
        }

        extrapolated
    }

    fn single_extrapolate(readings: &[i64]) -> Vec<i64> {
        readings
            .windows(2)
            .map(|slice| slice[1] - slice[0])
            .collect::<Vec<_>>()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let data = std::fs::read_to_string(&args[1])?;

    let series: Vec<Serie> = data
        .lines()
        .map(|l| Serie::parse(l))
        .collect::<Result<_, _>>()?;

    let sum = series.iter().fold(0, |acc, x| x.next_value() + acc);

    println!("part one {}", sum);

    let sum = series.iter().fold(0, |acc, x| x.previous_value() + acc);

    println!("part two {}", sum);

    Ok(())
}
