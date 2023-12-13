use std::u64;

use anyhow::Result;
use regex::Regex;

fn main() {
    let input = include_str!("input.in");
    let p1 = part1(input);
    dbg!(p1.unwrap());
    let p2 = part2(input);
    dbg!(p2.unwrap());
}

#[derive(Debug)]
struct Map {
    id: String,
    conversions: Vec<Conversion>,
}

#[derive(Debug)]
struct Conversion {
    destination: u64,
    source: u64,
    length: u64,
}

fn read_input(input: &str) -> Result<(Vec<u64>, Vec<Map>)> {
    let mut seeds: Vec<u64> = Vec::new();
    let mut maps: Vec<Map> = Vec::new();
    for line in input.lines() {
        if line.starts_with("seeds: ") {
            seeds = line
                .split(" ")
                .skip(1)
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
        } else if line.contains("map") {
            maps.push(Map {
                id: line.split(":").next().unwrap().to_string(),
                conversions: Vec::new(),
            });
        } else if Regex::new(r"^\d+(?: \d+){2}$").unwrap().is_match(line) {
            maps.last_mut().unwrap().conversions.push(Conversion {
                destination: line.split(" ").nth(0).unwrap().parse::<u64>().unwrap(),
                source: line.split(" ").nth(1).unwrap().parse::<u64>().unwrap(),
                length: line.split(" ").nth(2).unwrap().parse::<u64>().unwrap(),
            });
        } else {
            continue;
        }
    }
    Ok((seeds, maps))
}

impl Map {
    fn source_to_dest(&self, src: &u64) -> Result<u64> {
        for conversion in &self.conversions {
            if (conversion.source..=(conversion.source + conversion.length)).contains(src) {
                return Ok(conversion.destination + (src - conversion.source));
            }
        }
        Ok(*src)
    }
}

fn seed_to_loc(seed: &u64, maps: &Vec<Map>) -> Result<u64> {
    let mut cur = *seed;
    for map in maps {
        cur = map.source_to_dest(&cur)?;
    }
    Ok(cur)
}

fn part1(input: &str) -> Result<u64> {
    let (seeds, maps) = read_input(input)?;

    let min_location = seeds
        .iter()
        .map(|seed| seed_to_loc(seed, &maps).unwrap())
        .min()
        .unwrap();

    Ok(min_location)
}

fn part2(input: &str) -> Result<u64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        let expected = 35;
        let result = part1(include_str!("part1_ex.in"));
        println!("expected: {:?}, received {:?}", expected, result);
        assert_eq!(result.is_ok_and(|x| x == expected), true);
    }

    #[test]
    fn part2_ex() {
        let expected = 46;
        let result = part2(include_str!("part2_ex.in"));
        println!("expected: {:?}, received {:?}", expected, result);
        assert_eq!(result.is_ok_and(|x| x == expected), true);
    }
}
