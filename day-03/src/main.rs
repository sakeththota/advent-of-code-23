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

// returns true if the cell adjacent or diagonal to i,j is not a number and not a dot and false
// otherwise elegantly in rust
fn is_part_number(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Result<bool> {
    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for &(di, dj) in &offsets {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;

        if ni >= 0
            && nj >= 0
            && ni < grid.len() as i32
            && nj < grid[0].len() as i32
            && !grid[ni as usize][nj as usize].is_digit(10)
            && grid[ni as usize][nj as usize] != '.'
        {
            return Ok(true);
        }
    }

    Ok(false)
}

fn part1(input: &str) -> Result<usize> {
    let grid = read_grid(input)?;
    let mut sum: usize = 0;
    let mut current_number = String::from("");
    let mut check = false;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j].is_digit(10) {
                if is_part_number(&grid, i, j)? {
                    check = true;
                }
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

fn part2(input: &str) -> Result<usize> {
    let grid = read_grid(input)?;
    println!("{:?}", grid);
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
        let expected = 467835;
        let result = part2(include_str!("part2_ex.in"));
        println!("expected: {:?}, received {:?}", expected, result);
        assert_eq!(result.is_ok_and(|x| x == expected), true);
    }
}
