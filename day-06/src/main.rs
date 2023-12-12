use anyhow::Result;

fn main() {
    let input = include_str!("input.in");
    let p1 = part1(input);
    dbg!(p1.unwrap());
    let p2 = part2(input);
    dbg!(p2.unwrap());
}

// need to come back and clean this up gotta be a better way lmao
fn parse_races(input: &str) -> Result<(Vec<usize>, Vec<usize>)> {
    let (line1, line2) = input.split_once("\n").unwrap();
    let times: Vec<usize> = line1
        .strip_prefix("Time: ")
        .unwrap()
        .split_whitespace()
        .filter_map(|num| num.parse::<usize>().ok())
        .collect();
    let distances: Vec<usize> = line2
        .strip_prefix("Distance: ")
        .unwrap()
        .split_whitespace()
        .filter_map(|num| num.parse::<usize>().ok())
        .collect();
    Ok((times, distances))
}

fn parse_race((times, distances): (Vec<usize>, Vec<usize>)) -> Result<([usize; 1], [usize; 1])> {
    let times = [times
        .iter()
        .fold("".to_string(), |acc, x| acc + &x.to_string())
        .parse::<usize>()
        .unwrap()];
    let distances = [distances
        .iter()
        .fold("".to_string(), |acc, x| acc + &x.to_string())
        .parse::<usize>()
        .unwrap()];
    Ok((times, distances))
}

fn part1(input: &str) -> Result<usize> {
    let (times, distances) = parse_races(input)?;
    Ok(times.iter().enumerate().fold(1, |acc, (i, &x)| {
        acc * (1..=x).filter(|y| y * (x - y) > distances[i]).count()
    }))
}

fn part2(input: &str) -> Result<usize> {
    let (times, distances) = parse_race(parse_races(input)?)?;
    Ok(times.iter().enumerate().fold(1, |acc, (i, &x)| {
        acc * (1..=x).filter(|y| y * (x - y) > distances[i]).count()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        let expected = 288;
        let result = part1(include_str!("part1_ex.in"));
        println!("expected: {:?}, received {:?}", expected, result);
        assert_eq!(result.is_ok_and(|x| x == expected), true);
    }
}
