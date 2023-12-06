use::anyhow::Result;

fn main() {
    let input = include_str!("input.in");
    let p1 = part1(input);
    dbg!(p1);
    let p2 = part2(input);
    dbg!(p2);
}

fn part1(input: &str) -> Result<usize> {
    let mut sum: usize = 0;
    for line in input.lines() {
        let (winners, numbers) = line[line.find(':').unwrap()+2..].split_once(" | ").unwrap();
        let winners: Vec<&str> = winners.split_whitespace().collect();
        let numbers: Vec<&str> = numbers.split_whitespace().collect();
        let mut num_wins = 0;
        for num in numbers {
            if winners.contains(&num) {
                num_wins += 1;
            }
        }
        if num_wins > 0 {
            sum += usize::pow(2, num_wins-1);
        }
    }
    Ok(sum)
}

fn part2(input: &str) -> Result<usize> {
    let mut sum: usize = 0;
    for line in input.lines() {
        let (winners, numbers) = line[line.find(':').unwrap()+2..].split_once(" | ").unwrap();
        let winners: Vec<&str> = winners.split_whitespace().collect();
        let numbers: Vec<&str> = numbers.split_whitespace().collect();
        let mut num_wins = 0;
        for num in numbers {
            if winners.contains(&num) {
                num_wins += 1;
            }
        }
        if num_wins > 0 {
            sum += usize::pow(2, num_wins-1);
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        let expected = 13;
        let result = part1(include_str!("part1_ex.in"));
        println!("expected: {:?}, received {:?}", expected, result);
        assert_eq!(result.is_ok_and(|x| x == expected), true);
    }

    #[test]
    fn part2_ex() {
        let expected = 30;
        let result = part1(include_str!("part1_ex.in"));
        println!("expected: {:?}, received {:?}", expected, result);
        assert_eq!(result.is_ok_and(|x| x == expected), true);
    }
}
