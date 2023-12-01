// Advent of code: Day 1
// Author: @alisinabh

use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<_> = env::args().collect();
    let file = read_to_string(&args[1]).unwrap();

    let mut sum: i32 = 0;

    for line in file.lines() {
        sum = sum + extract_value(line).unwrap();
    }

    println!("The sum is {}", sum);
}

fn extract_value(line: &str) -> Result<i32, std::num::ParseIntError> {
    let mut cursor: usize = 0;
    let line = replace_literal_digits(line);
    let chars_count = line.chars().count();

    while cursor < chars_count {
        match line.chars().nth(cursor) {
            Some(c) if c.is_digit(10) => break,
            _ => (),
        }
        cursor = cursor + 1;
    }

    let mut rcursor: usize = chars_count - 1;

    loop {
        match line.chars().nth(rcursor) {
            Some(c) if c.is_digit(10) => break,
            _ => (),
        }

        if rcursor == 0 {
            break;
        } else {
            rcursor = rcursor - 1;
        }
    }

    let number_str = format!(
        "{}{}",
        line.chars().nth(cursor).unwrap(),
        line.chars().nth(rcursor).unwrap()
    );

    number_str.parse()
}

fn replace_literal_digits(line: &str) -> String {
    let mut normal_line = String::new();
    let mut i = 0;
    let chars_count = line.chars().count();

    while i < chars_count {
        let char_and_len: (char, usize) = if line[i..].starts_with("one") {
            ('1', "one".len())
        } else if line[i..].starts_with("two") {
            ('2', "two".len())
        } else if line[i..].starts_with("three") {
            ('3', "three".len())
        } else if line[i..].starts_with("four") {
            ('4', "four".len())
        } else if line[i..].starts_with("five") {
            ('5', "five".len())
        } else if line[i..].starts_with("six") {
            ('6', "six".len())
        } else if line[i..].starts_with("seven") {
            ('7', "seven".len())
        } else if line[i..].starts_with("eight") {
            ('8', "eight".len())
        } else if line[i..].starts_with("nine") {
            ('9', "nine".len())
        } else {
            (line.chars().nth(i).unwrap(), 1)
        };

        normal_line.push(char_and_len.0);
        i = i + char_and_len.1;
    }

    normal_line
}
