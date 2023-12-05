use ::anyhow::Result;

fn main() {
    let input = include_str!("part1_ex.in");
    let res = part1(input).unwrap();
    dbg!(res);
}

fn read_grid(input: &str) -> Result<Vec<Vec<char>>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    Ok(grid)
}

fn is_part_number(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Result<bool> {
    if i > 0 && grid[i - 1][j] != '.'
        || i < grid.len() - 1 && grid[i + 1][j] != '.'
        || j > 0 && grid[i][j - 1] != '.'
        || j < grid[0].len() - 1 && grid[i][j + 1] != '.'
        || i > 0 && j > 0 && grid[i - 1][j - 1] != '.'
        || i > 0 && j < grid[0].len() - 1 && grid[i - 1][j + 1] != '.'
        || i < grid.len() - 1 && j > 0 && grid[i + 1][j - 1] != '.'
        || i < grid.len() - 1 && j < grid[0].len() - 1 && grid[i + 1][j + 1] != '.'
    {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn part1(input: &str) -> Result<usize> {
    let grid = read_grid(input)?;
    let mut sum: usize = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            println!("{} {} {}", i, j, grid[i][j]);
            if grid[i][j].is_digit(10) && is_part_number(&grid, i, j)? {
                let val = grid[i][j].to_digit(10).unwrap() as usize;
                println!("{} {} {}", i, j, val);
                sum += val;
            }
        }
    }
    Ok(sum)
}

fn part2(input: &str) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1_ex() {
        assert_eq!(true, true);
    }

    #[test]
    fn part2_ex() {
        assert_eq!(true, true);
    }
}
