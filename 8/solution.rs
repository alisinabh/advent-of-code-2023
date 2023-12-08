// Advent of code: Day 7
// Author: @alisinabh

use std::collections::HashMap;
use std::env;
use std::error::Error;

fn next_pos<'a>(
    instructions: &'a [Direction],
    map: &'a HashMap<String, (String, String)>,
    pos: &'a str,
    step: usize,
) -> Result<&'a str, Box<dyn Error>> {
    let current_map_node = map.get(pos).ok_or("map node not found!")?;

    match instructions[step % instructions.len()] {
        Direction::Left => Ok(&current_map_node.0),
        Direction::Right => Ok(&current_map_node.1),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let data = std::fs::read_to_string(&args[2])?;

    let part_one = args[1] == "1";
    let part_two = args[1] == "2";

    let (instructions, map) = parse(&data)?;

    if part_one {
        let mut pos: String = "AAA".into();
        let mut step = 0;

        while pos != "ZZZ" {
            pos = next_pos(&instructions, &map, &pos, step)?.into();
            step += 1;
        }

        println!("part one took {} steps", step);
    }

    if part_two {
        let mut pos: Vec<String> = Vec::new();

        for m in map.keys() {
            if m.ends_with("A") {
                pos.push(m.to_string());
            }
        }

        let z_min_steps: Vec<usize> = pos
            .iter()
            .map(|x| {
                let mut p: &str = x;
                let mut i: usize = 0;
                loop {
                    p = &next_pos(&instructions, &map, &p, i).unwrap();
                    i += 1;
                    if p.ends_with("Z") {
                        break;
                    }
                }
                i
            })
            .collect::<Vec<_>>();

        let gcd = z_min_steps
            .iter()
            .cloned()
            .reduce(|acc, x| gcd(x, acc))
            .unwrap();

        let steps: usize = z_min_steps
            .iter()
            .cloned()
            .reduce(|acc, x| x * acc / gcd)
            .unwrap();

        println!("part twp steps {}", steps);
    }

    Ok(())
}

fn gcd(x: usize, y: usize) -> usize {
    let mut a = x;
    let mut b = y;

    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }

    a
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: &char) -> Result<Self, Box<dyn Error>> {
        match *c {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err("invalid instruction".into()),
        }
    }
}

fn parse(
    data: &str,
) -> Result<(Vec<Direction>, HashMap<String, (String, String)>), Box<dyn Error>> {
    let mut lines = data.lines();

    let instructions_line = lines.next().ok_or("instruction not found")?;

    let instructions: Vec<Direction> = instructions_line
        .chars()
        .map(|c| Direction::from_char(&c))
        .collect::<Result<_, _>>()?;

    _ = lines.next();

    let mut map: HashMap<_, _> = HashMap::new();

    while let Some(l) = lines.next() {
        let [pos, next_pos] = l.split('=').collect::<Vec<_>>()[..] else {return Err("invalid pos".into())};

        let pos = pos.trim();
        let mut next_pos = next_pos
            .split(',')
            .map(|c| c.replace("(", "").replace(")", "").replace(" ", ""));

        map.insert(
            pos.into(),
            (
                next_pos.next().ok_or("next pos not foun")?,
                next_pos.next().ok_or("next pos not found")?,
            ),
        );
    }

    Ok((instructions, map))
}
