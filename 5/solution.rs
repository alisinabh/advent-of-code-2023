// Advent of code: Day 5
// Author: @alisinabh

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::read_to_string;
use std::ops::Range;

#[derive(Debug)]
struct SeedData {
    seeds: Vec<u64>,
    ranges: HashMap<(String, String), ConversionRange>,
}

#[derive(Debug)]
struct ConversionRange {
    ranges: Vec<(Range<u64>, i64)>,
}

const PATH: [&str; 8] = [
    "seed",
    "soil",
    "fertilizer",
    "water",
    "light",
    "temperature",
    "humidity",
    "location",
];

impl SeedData {
    fn parse(data: String) -> Result<SeedData, Box<dyn Error>> {
        let mut lines = data.lines();

        let Some(seeds_line) = lines.next() else { return Err("missing seeds line".into()) };
        let Some(seeds) = seeds_line.split(':').last() else { return Err("bad seeds line".into()) };

        let seeds = seeds
            .split_whitespace()
            .map(|x| x.parse())
            .collect::<Result<_, _>>()?;

        let mut ranges = HashMap::new();

        _ = lines.next();

        while let Some(map_line) = lines.next() {
            let [src, "to", dst] = map_line
                .split_whitespace()
                .next()
                .unwrap()
                .split('-')
                .collect::<Vec<_>>()[..] else {return Err("bad map line".into())};

            let mut conversion_ranges = ConversionRange::new();

            loop {
                match lines.next() {
                    Some(s) if !s.trim().is_empty() => conversion_ranges.add_range(s)?,
                    _ => break,
                };
            }

            ranges.insert((src.to_string(), dst.to_string()), conversion_ranges);
        }

        Ok(SeedData {
            seeds: seeds,
            ranges: ranges,
        })
    }

    fn traverse(&self, seed: u64) -> u64 {
        let mut i = 0;
        let mut value = seed;

        while i + 1 < PATH.len() {
            value = self.ranges[&(PATH[i].into(), PATH[i + 1].into())].get_next_value(value);
            i += 1;
        }

        value
    }

    fn find_lowest_location(&self) -> Option<u64> {
        self.seeds.iter().map(|&s| self.traverse(s)).min()
    }
}

impl ConversionRange {
    fn new() -> ConversionRange {
        ConversionRange { ranges: Vec::new() }
    }

    fn add_range(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let [dst_range, src_range, len] = line
            .split_whitespace()
            .map(|x| x.parse())
            .collect::<Result<Vec<u64>, _>>()?[..] else { return Err("invalid range".into()) };

        self.ranges.push((
            src_range..src_range + len,
            dst_range as i64 - src_range as i64,
        ));

        Ok(())
    }

    fn get_next_value(&self, value: u64) -> u64 {
        match self
            .ranges
            .iter()
            .find(|&(range, _)| range.contains(&value))
        {
            Some(&(_, diff)) => (value as i64 + diff as i64) as u64,
            None => value,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let file = read_to_string(&args[1])?;

    let seed_data = SeedData::parse(file)?;

    println!("{:?}", seed_data);

    println!(
        "{}",
        seed_data
            .find_lowest_location()
            .ok_or("location not found")?
    );

    Ok(())
}
