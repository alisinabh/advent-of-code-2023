// Advent of code: Day 3
// Author: @alisinabh

use std::env;
use std::fs::read_to_string;

#[derive(Debug)]
struct Engine {
    part_numbers: Vec<PartNumber>,
    symbols_matrix: Vec<(usize, usize)>,
}

#[derive(Debug)]
struct PartNumber {
    value: i64,
    position: (usize, usize, usize),
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let file = read_to_string(&args[1]).unwrap();

    let mut engine = Engine::from_file(file);

    // let mut sum: i32 = sum_part_numbers(lines);

    println!("Engine {:#?}", engine);

    println!("The sum is {}", engine.sum_part_numbers());
}

impl Engine {
    fn from_file(file: String) -> Engine {
        let mut part_numbers: Vec<PartNumber> = Vec::new();
        let mut symbols_matrix: Vec<(usize, usize)> = Vec::new();
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
                    Some(_) => symbols_matrix.push((i, j)),
                    None => break,
                }
            }
            i = i + 1;
        }

        Engine {
            part_numbers: part_numbers,
            symbols_matrix: symbols_matrix,
        }
    }

    fn sum_part_numbers(&mut self) -> i64 {
        let mut sum = 0;

        for p in &self.part_numbers {
            let (oi, oj, count) = p.position;
            match self
                .symbols_matrix
                .iter()
                .find(|&(i, j)| *j <= oj + 1 && *j + 1 >= oj && *i + 1 >= oi && oi + count >= *i)
            {
                Some(_) => sum = sum + p.value,
                None => {
                    println!("{:?}", p);
                }
            }
        }

        sum
    }
}
