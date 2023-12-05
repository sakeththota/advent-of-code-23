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

// returns true if the cell adjacent or diagonal to i,j is not a number and not a dot and false
// otherwise elegantly in rust
fn is_part_number(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Result<bool> {
    let offsets = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), /*(i, j)*/ (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    for &(di, dj) in &offsets {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;

        if ni >= 0 && nj >= 0 &&
           ni < grid.len() as i32 && nj < grid[0].len() as i32 &&
           !grid[ni as usize][nj as usize].is_digit(10) &&
           grid[ni as usize][nj as usize] != '.' {
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
                check = if is_part_number(&grid,i,j)? {true} else {check};
                current_number.push(grid[i][j]);
            } else {
                if check { sum += current_number.parse::<usize>()?; }
                current_number = String::from("");
                check = false;
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
