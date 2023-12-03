// Advent of code: Day 3
// Author: @alisinabh

use std::env;
use std::fs::read_to_string;

#[derive(Debug)]
struct Engine {
    part_numbers: Vec<PartNumber>,
    symbols_matrix: Vec<(usize, usize)>,
    gears: Vec<(usize, usize)>,
}

#[derive(Debug)]
struct PartNumber {
    value: i64,
    position: (usize, usize, usize),
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let file = read_to_string(&args[1]).unwrap();

    let engine = Engine::from_file(file);

    // println!("Engine {:?}", engine);

    println!("The sum is {}", engine.sum_part_numbers());

    println!("The gear ratio sum is {}", engine.sum_gear_ratios());
}

impl Engine {
    fn from_file(file: String) -> Engine {
        let mut part_numbers: Vec<PartNumber> = Vec::new();
        let mut symbols_matrix: Vec<(usize, usize)> = Vec::new();
        let mut gears: Vec<(usize, usize)> = Vec::new();

        let mut chars = file.chars();

        let mut i = 0;
        let mut j = 0;

        let mut part_temp = String::new();

        loop {
            let char = chars.next();

            if char.is_some() && char.unwrap().is_digit(10) {
                part_temp.push(char.unwrap());
            } else {
                if part_temp.len() > 0 {
                    let count = part_temp.chars().count();
                    part_numbers.push(PartNumber {
                        value: part_temp.parse().unwrap(),
                        position: (i - count, j, count),
                    });
                }
                part_temp.truncate(0);

                match char {
                    Some('.') => {}
                    Some('\n') => {
                        j = j + 1;
                        i = 0;
                        continue;
                    }
                    Some('\r') => continue,
                    Some(sym) => {
                        if sym == '*' {
                            gears.push((i, j));
                        }
                        symbols_matrix.push((i, j))
                    }
                    None => break,
                }
            }
            i = i + 1;
        }

        Engine {
            part_numbers: part_numbers,
            symbols_matrix: symbols_matrix,
            gears: gears,
        }
    }

    fn sum_part_numbers(&self) -> i64 {
        let mut sum = 0;

        for p in &self.part_numbers {
            let (oi, oj, count) = p.position;
            match self
                .symbols_matrix
                .iter()
                .find(|&(i, j)| *j <= oj + 1 && *j + 1 >= oj && *i + 1 >= oi && oi + count >= *i)
            {
                Some(_) => sum = sum + p.value,
                None => {}
            }
        }

        sum
    }

    fn sum_gear_ratios(&self) -> i64 {
        self.gears.iter().fold(0, |acc, (gi, gj)| {
            let gear_values: Vec<_> = self
                .part_numbers
                .iter()
                .filter(|p| {
                    let (i, j, count) = p.position;
                    j <= *gj + 1 && j + 1 >= *gj && *gi + 1 >= i && *gi <= i + count
                })
                .collect();

            // println!("gears {:?}", gear_values);

            match gear_values[..] {
                [x, y] => x.value * y.value + acc,
                _ => acc,
            }
        })
    }
}
