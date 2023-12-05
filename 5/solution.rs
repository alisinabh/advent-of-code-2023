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

    fn next_edge_diff(&self, start_seed: u64, end_seed: u64) -> u64 {
        let c_ranges: Vec<_> = PATH
            .windows(2)
            .map(|path| &self.ranges[&(path[0].into(), path[1].into())])
            .collect();

        let mut min_change: u64 = end_seed - start_seed;
        let mut value = start_seed;

        for c in c_ranges {
            let range = c
                .get_related_range(value)
                .or_else(|| c.get_next_range(value));

            if let Some(&(ref range, diff)) = range {
                min_change = std::cmp::min(range.end - value, min_change);
                value = (value as i64 + diff) as u64;
            }
        }

        start_seed + min_change
    }

    fn find_lowest_location(&self) -> Option<u64> {
        self.seeds.iter().map(|&s| self.traverse(s)).min()
    }

    fn find_lowest_location_using_ranges(&self) -> u64 {
        let mut location = u64::MAX;

        let mut i = 0;

        while i < self.seeds.len() {
            let mut seed = self.seeds[i];
            let max_seed = seed + self.seeds[i + 1];

            loop {
                match self.traverse(seed) {
                    s if s < location => location = s,
                    _ => {
                        seed = self.next_edge_diff(seed, max_seed);
                        if seed >= max_seed {
                            break;
                        }
                    }
                }
            }

            i += 2;
        }

        location
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

    fn get_related_range(&self, value: u64) -> Option<&(Range<u64>, i64)> {
        self.ranges
            .iter()
            .find(|&(range, _)| range.contains(&value))
    }

    fn get_next_range(&self, start: u64) -> Option<&(Range<u64>, i64)> {
        self.ranges
            .iter()
            .filter(|&(range, _)| range.start > start)
            .min_by(|&(range_1, _), &(range_2, _)| range_1.start.cmp(&range_2.start))
    }

    fn get_next_value(&self, value: u64) -> u64 {
        match self.get_related_range(value) {
            Some(&(_, diff)) => (value as i64 + diff as i64) as u64,
            None => value,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let file = read_to_string(&args[1])?;

    let seed_data = SeedData::parse(file)?;

    println!(
        "part one: {}",
        seed_data
            .find_lowest_location()
            .ok_or("location not found")?
    );

    println!(
        "part two: {}",
        seed_data.find_lowest_location_using_ranges()
    );

    Ok(())
}
