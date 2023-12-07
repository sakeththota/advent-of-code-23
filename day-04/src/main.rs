use ::anyhow::Result;

fn main() {
    let input = include_str!("input.in");
    let p1 = part1(input);
    dbg!(p1.unwrap());
    let p2 = part2(input);
    dbg!(p2.unwrap());
}

fn count_wins((winners, scratchcard): (Vec<&str>, Vec<&str>)) -> Result<usize> {
    Ok(scratchcard.iter().filter(|x| winners.contains(&x)).count())
}

fn parse_card(card: &str) -> Result<(Vec<&str>, Vec<&str>)> {
    let (winners, scratchcard) = card[card.find(":").unwrap() + 2..]
        .split_once(" | ")
        .unwrap();
    Ok((
        winners.split_whitespace().collect(),
        scratchcard.split_whitespace().collect(),
    ))
}

fn part1(input: &str) -> Result<usize> {
    let mut sum: usize = 0;
    for line in input.lines() {
        let num_wins = count_wins(parse_card(line)?)? as u32;
        if num_wins > 0 {
            sum += 1 << (num_wins - 1);
        }
    }
    Ok(sum)
}

fn part2(input: &str) -> Result<usize> {
    let mut num_wins: Vec<usize> = Vec::new();
    for line in input.lines() {
        num_wins.push(count_wins(parse_card(line)?)?);
    }
    let mut dp: Vec<usize> = vec![1; num_wins.len()];
    for i in (0..num_wins.len()).rev() {
        if num_wins[i] > 0 {
            dp[i] = 1
                + (i + 1..i + num_wins[i] + 1)
                    .into_iter()
                    .map(|x| dp[x])
                    .sum::<usize>();
        }
    }
    Ok(dp.iter().sum())
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
        let result = part2(include_str!("part2_ex.in"));
        println!("expected: {:?}, received {:?}", expected, result);
        assert_eq!(result.is_ok_and(|x| x == expected), true);
    }
}
