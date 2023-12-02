// Advent of code: Day 2
// Author: @alisinabh

use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<_> = env::args().collect();
    let file = read_to_string(&args[1]).unwrap();

    let mut sum: i32 = 0;

    for line in file.lines() {
        if let Some(id) = game_id_if_possible(line) {
            sum = sum + id;
        }
    }

    println!("The sum is {}", sum);

    sum = 0;

    for line in file.lines() {
        sum = sum + calculate_game_power(line);
    }

    println!("The power sum is {}", sum);
}

fn calculate_game_power(line: &str) -> i32 {
    let column_indx = line.find(':').unwrap();

    let mut draws = (&line[column_indx + 1..]).split(';').map(|d| d.split(','));

    let mut maximums: HashMap<&str, i32> = HashMap::with_capacity(3);

    while let Some(mut details) = draws.next() {
        while let Some(item) = details.next() {
            let [x, color] = item.trim().split(' ').collect::<Vec<_>>()[..] else {panic!("Invalid data")};
            let x: i32 = x.parse::<i32>().unwrap();

            match maximums.get(color) {
                Some(&max) if max > x => None,
                _ => maximums.insert(color, x),
            };
        }
    }

    maximums.values().fold(1, |acc, x| acc * x)
}

fn game_id_if_possible(line: &str) -> Option<i32> {
    let column_indx = line.find(':')?;
    let game_id: i32 = (&line[5..column_indx]).parse().ok()?;

    let mut draws = (&line[column_indx + 1..]).split(';').map(|d| d.split(','));

    while let Some(mut details) = draws.next() {
        while let Some(item) = details.next() {
            match item.trim().split(' ').collect::<Vec<_>>()[..] {
                [x, "red"] if x.parse::<i32>().unwrap() > 12 => return None,
                [x, "green"] if x.parse::<i32>().unwrap() > 13 => return None,
                [x, "blue"] if x.parse::<i32>().unwrap() > 14 => return None,
                [_, _] => (),
                _ => panic!("unmatch {}", item),
            }
        }
    }

    Some(game_id)
}
