use anyhow::Result;
use std::collections::HashMap;
fn main() {
    let input = include_str!("input.in");
    let p1 = part1(input);
    dbg!(p1.unwrap());
    let p2 = part2(input);
    dbg!(p2.unwrap());
}

fn part1(input: &str) -> Result<u32> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
        let first: u32 = digits.first().unwrap().to_digit(10).unwrap();
        let last: u32 = digits.last().unwrap().to_digit(10).unwrap();
        sum += first * 10 + last;
    }
    Ok(sum)
}

fn part2(input: &str) -> Result<u32> {
    let numbers: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("ten", 10),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("10", 10),
    ]);

    let mut sum: u32 = 0;
    let mut digits = Vec::<u32>::new();
    for line in input.lines() {
        for i in 0..line.len() {
            for (key, val) in numbers.iter() {
                if line[i..].starts_with(key) {
                    digits.push(*val);
                }
            }
        }
        let first: u32 = *digits.first().unwrap();
        let last: u32 = *digits.last().unwrap();
        sum += first * 10 + last;
        digits.clear()
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        let expected = 142;
        let result = part1(include_str!("part1_ex.in"));
        println!("expected: {:?}, received {:?}", expected, result);
        assert_eq!(result.is_ok_and(|x| x == expected), true);
    }

    #[test]
    fn part2_ex() {
        let expected = 281;
        let result = part2(include_str!("part2_ex.in"));
        println!("expected: {:?}, received {:?}", expected, result);
        assert_eq!(result.is_ok_and(|x| x == expected), true);
    }
}
