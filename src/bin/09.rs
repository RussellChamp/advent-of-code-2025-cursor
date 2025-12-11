advent_of_code::solution!(9);

use itertools::Itertools;

fn parse_coords(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let (x, y) = line.split_once(',')?;
            Some((x.trim().parse().ok()?, y.trim().parse().ok()?))
        })
        .collect()
}

fn rectangle_area(p1: (i64, i64), p2: (i64, i64)) -> u64 {
    let width = (p2.0 - p1.0).abs() + 1;
    let height = (p2.1 - p1.1).abs() + 1;
    (width * height) as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let coords = parse_coords(input);

    coords
        .iter()
        .tuple_combinations()
        .map(|(&p1, &p2)| rectangle_area(p1, p2))
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
