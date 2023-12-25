use anyhow::Result;

fn main() {
    println!("Hello, world!");
}

fn extrapolate(history: Vec<usize>) -> Result<usize> {
    Ok(0)
}

fn part1(input: &str) -> Result<usize> {
    let mut sum = 0;
    for line in input.lines() {
        let history = line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        sum += extrapolate(history)?;
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let expected = 114;
        let result = part1(include_str!("example.in"));
        println!("expected: {:?}, received {:?}", expected, result);
        assert_eq!(result.is_ok_and(|x| x == expected), true);
    }
}
