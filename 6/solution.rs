// Advent of code: Day 6
// Author: @alisinabh

use std::env;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug)]
struct RaceHistory {
    races: Vec<Race>,
}

impl RaceHistory {
    fn parse(file: &str) -> Result<RaceHistory, Box<dyn Error>> {
        let mut lines = file
            .lines()
            .map(|l| l.split_whitespace().collect::<Vec<_>>());

        let binding = lines
            .find(|x| x[0] == "Time:")
            .ok_or("time entries not found")?;
        let times_line = binding.iter();

        let binding = lines
            .find(|x| x[0] == "Distance:")
            .ok_or("distance entries not found")?;
        let distances_line = binding.iter();

        let times: Vec<_> = times_line
            .skip(1)
            .map(|&t| t.parse::<u64>())
            .collect::<Result<_, _>>()?;
        let distances: Vec<_> = distances_line
            .skip(1)
            .map(|&d| d.parse::<u64>())
            .collect::<Result<_, _>>()?;

        let races = times
            .iter()
            .zip(distances.iter())
            .map(|(&a, &b)| Race::new(a, b))
            .collect::<Vec<Race>>();

        Ok(RaceHistory { races })
    }
}

#[derive(Debug)]
struct Race {
    time: u64,
    best_distance: u64,
}

impl Race {
    fn new(time: u64, best_distance: u64) -> Self {
        Self {
            time: time,
            best_distance: best_distance,
        }
    }

    fn parse_single(file: &str) -> Result<Self, Box<dyn Error>> {
        let mut lines = file.lines().map(|l| l.split(':').collect::<Vec<_>>());

        let time = lines.find(|x| x[0] == "Time").ok_or("time not found")?[1]
            .replace(" ", "")
            .parse()?;

        let distance = lines
            .find(|x| x[0] == "Distance")
            .ok_or("distance not found")?[1]
            .replace(" ", "")
            .parse()?;

        Ok(Race::new(time, distance))
    }

    fn record_breaking_possibility_count(&self) -> u64 {
        let optimal_speed = self.time / 2;

        let mut beating_scenarios = 0;

        let mut speed = optimal_speed;

        // Try with higher than optimal speeds until cut-off
        loop {
            let distance = speed * (self.time - speed);

            if distance <= self.best_distance {
                break;
            }

            beating_scenarios += 1;
            speed += 1;
        }

        // -1 since we have already counter optimal speed in the abov loop
        speed = optimal_speed - 1;

        // Try with lower than optimal speeds until cut-off
        loop {
            let distance = speed * (self.time - speed);

            if distance <= self.best_distance {
                break;
            }

            beating_scenarios += 1;
            speed -= 1;
        }

        beating_scenarios
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let file = read_to_string(&args[1])?;

    let race_history = RaceHistory::parse(&file)?;

    let beating_scenarios = race_history
        .races
        .iter()
        .map(|r| r.record_breaking_possibility_count())
        .fold(1, |acc, c| c * acc);

    println!("part one: {}", beating_scenarios);

    let race = Race::parse_single(&file)?;

    println!("part two: {}", race.record_breaking_possibility_count());

    Ok(())
}
