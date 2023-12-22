use anyhow::Result;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.in");
    let p1 = part1(input);
    dbg!(p1.unwrap());
}

fn part1(input: &str) -> Result<usize> {
    let (path, map) = input.split_once("\n\n").unwrap();

    let map: HashMap<&str, (&str, &str)> = map
        .lines()
        .map(|line| {
            let (parent, children) = line.split_once(" = ").unwrap();
            let (l_child, r_child) = children
                .strip_prefix("(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split_once(", ")
                .unwrap();
            (parent, (l_child, r_child))
        })
        .collect();

    let mut path = path.chars().cycle();
    let mut curr = "AAA";
    let mut steps = 0;

    while curr != "ZZZ" {
        let (l_child, r_child) = map.get(curr).unwrap();
        curr = match path.next().unwrap() {
            'L' => l_child,
            'R' => r_child,
            _ => panic!("Invalid direction"),
        };
        steps += 1;
    }

    Ok(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        let expected = 2;
        let result = part1(include_str!("example.in"));
        println!("expected: {:?}, received {:?}", expected, result);
        assert_eq!(result.is_ok_and(|x| x == expected), true);
    }

    #[test]
    fn part1_ex2() {
        let expected = 6;
        let result = part1(include_str!("example2.in"));
        println!("expected: {:?}, received {:?}", expected, result);
        assert_eq!(result.is_ok_and(|x| x == expected), true);
    }
}
