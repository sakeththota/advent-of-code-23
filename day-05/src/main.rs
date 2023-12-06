use ::anyhow::Result;

fn main() {
    let input = include_str!("input.in");
    let res = part1(input).unwrap();
    dbg!(res);
}

fn part1(input: &str) -> Result<usize> {
    Ok(0)
}

fn part2(input: &str) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        let expected = 4361;
        let result = part1(include_str!("part1_ex.in"));
        println!("expected: {:?}, received {:?}", expected, result);
        assert_eq!(result.is_ok_and(|x| x == expected), true);
    }

    #[test]
    fn part2_ex() {
        let expected = 4361;
        let result = part1(include_str!("part1_ex.in"));
        println!("expected: {:?}, received {:?}", expected, result);
        assert_eq!(result.is_ok_and(|x| x == expected), true);
    }
}
