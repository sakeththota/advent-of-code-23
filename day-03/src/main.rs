use std::collections::HashSet;

use ::anyhow::Result;

fn main() {
    let input = include_str!("input.in");
    let p1 = part1(input).unwrap();
    dbg!(p1);
    let p2 = part2(input).unwrap();
    dbg!(p2);
}

fn read_grid(input: &str) -> Result<Vec<Vec<char>>> {
    Ok(input
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>())
}

fn get_adj_val(grid: &Vec<Vec<char>>, i: usize, j: usize, (x, y): &(i32, i32)) -> Result<char> {
    let new_i = i as i32 + x;
    let new_j = j as i32 + y;
    if new_i >= 0 && new_i < grid.len() as i32 && new_j >= 0 && new_j < grid[0].len() as i32 {
        return Ok(grid[new_i as usize][new_j as usize]);
    } else {
        return Err(anyhow::anyhow!("Out of bounds"));
    }
}

fn is_part_number(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Result<bool> {
    let offsets: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    Ok(offsets.iter().fold(false, |acc, offset| {
        let val = get_adj_val(grid, i, j, offset);
        match val {
            Ok(val) => acc || (!val.is_digit(10) && val != '.'),
            Err(_e) => acc,
        }
    }))
}

fn part1(input: &str) -> Result<usize> {
    let grid = read_grid(input)?;
    let mut sum: usize = 0;
    let mut current_number = String::from("");
    let mut check = false;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j].is_digit(10) {
                check = check || is_part_number(&grid, i, j)?;
                current_number.push(grid[i][j]);
            } else {
                if check {
                    sum += current_number.parse::<usize>()?;
                }
                current_number = String::from("");
                check = false;
            }
        }
    }
    Ok(sum)
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct PartNumber {
    value: String,
    y: usize,
    x1: usize,
    x2: usize,
}

fn get_part_number(part_numbers: &Vec<PartNumber>, i: usize, j: usize) -> Result<&PartNumber> {
    for part_number in part_numbers {
        if part_number.y == i && (part_number.x1..part_number.x2 + 1).contains(&j) {
            return Ok(part_number);
        }
    }
    Err(anyhow::anyhow!("unknown error occurred"))
}

fn get_gear_ratio(
    part_numbers: &Vec<PartNumber>,
    grid: &Vec<Vec<char>>,
    i: usize,
    j: usize,
) -> Result<usize> {
    let mut parts: HashSet<&PartNumber> = HashSet::new();
    let offsets: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for (x, y) in offsets {
        let val = get_adj_val(grid, i, j, &(x, y));
        match val {
            Ok(val) => {
                println!("{}", val);
                if val.is_digit(10) {
                    let out: &PartNumber = get_part_number(
                        &part_numbers,
                        (i as i32 + x) as usize,
                        (j as i32 + y) as usize,
                    )?;
                    println!("{:?}", out);
                    parts.insert(out);
                }
            }
            Err(_e) => continue,
        }
    }

    if parts.len() == 2 {
        let mut product = 1;
        println!("*: ");
        for part in parts.iter() {
            println!("{:?}", part);
            product *= part.value.parse::<usize>()?;
        }
        Ok(product)
    } else {
        Ok(0)
    }
}

fn part2(input: &str) -> Result<usize> {
    let grid = read_grid(input)?;
    let mut part_numbers: Vec<PartNumber> = vec![];
    let mut current_number = String::from("");
    let mut check = false;
    for i in 0..grid.len() {
        let mut start = -1;
        let mut end = -1;
        for j in 0..grid[0].len() {
            if grid[i][j].is_digit(10) {
                if start == -1 {
                    start = j as isize;
                    end = j as isize;
                } else {
                    end = j as isize;
                }
                current_number.push(grid[i][j]);
                check = check || is_part_number(&grid, i, j)?;
            } else {
                if check {
                    part_numbers.push(PartNumber {
                        value: current_number.clone(),
                        y: if j == 0 { i - 1 } else { i },
                        x1: start as usize,
                        x2: end as usize,
                    });
                }
                current_number = String::from("");
                check = false;
                start = -1;
            }
        }
    }
    println!("{:?}", part_numbers);
    let mut sum_gear_ratios = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '*' {
                sum_gear_ratios += get_gear_ratio(&part_numbers, &grid, i, j)?;
            }
        }
    }
    Ok(sum_gear_ratios)
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
        let expected = 467835;
        let result = part2(include_str!("part2_ex.in"));
        println!("expected: {:?}, received {:?}", expected, result);
        assert_eq!(result.is_ok_and(|x| x == expected), true);
    }
}
