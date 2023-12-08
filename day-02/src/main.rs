use ::anyhow::Result;

fn main() {
    let input = include_str!("input.in");
    let p1 = part1(input);
    dbg!(p1.unwrap());
    let p2 = part2(input);
    dbg!(p2.unwrap());
}

fn part1(input: &str) -> Result<usize> {
    let mut sum: usize = 0;
    'game_loop: for (id, line) in input.lines().enumerate() {
        let game = line[line.find(':').unwrap() + 2..].split("; ");
        for draw in game {
            for pair in draw.split(", ") {
                let (num, color) = pair.split_once(" ").unwrap();
                let num: usize = num.parse::<usize>().unwrap();
                let is_valid = match color {
                    "red" => num <= 12,
                    "green" => num <= 13,
                    "blue" => num <= 14,
                    _ => panic!("should never reach here"),
                };
                if !is_valid {
                    continue 'game_loop;
                }
            }
        }
        sum += id + 1;
    }

    Ok(sum)
}

fn part2(input: &str) -> Result<usize> {
    let mut sum: usize = 0;
    for (id, line) in input.lines().enumerate() {
        let game = line[line.find(':').unwrap() + 2..].split("; ");
        let (mut max_r, mut max_g, mut max_b) = (0, 0, 0);
        for draw in game {
            for pair in draw.split(", ") {
                let (num, color) = pair.split_once(" ").unwrap();
                let num: usize = num.parse::<usize>().unwrap();
                match color {
                    "red" => {
                        if num > max_r {
                            max_r = num;
                        }
                    }
                    "green" => {
                        if num > max_g {
                            max_g = num;
                        }
                    }
                    "blue" => {
                        if num > max_b {
                            max_b = num;
                        }
                    }
                    _ => panic!("should never reach here"),
                };
            }
        }
        let power = max_r * max_g * max_b;
        sum += power
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        let expected = 8;
        let result = part1(include_str!("part1_ex.in"));
        println!("expected: {:?}, received {:?}", expected, result);
        assert_eq!(result.is_ok_and(|x| x == expected), true);
    }

    #[test]
    fn part2_ex() {
        let expected = 2286;
        let result = part2(include_str!("part2_ex.in"));
        println!("expected: {:?}, received {:?}", expected, result);
        assert_eq!(result.is_ok_and(|x| x == expected), true);
    }
}
