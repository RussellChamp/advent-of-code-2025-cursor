use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(2);

/// Parse a range string like "11-22" into (start, end)
fn parse_range(s: &str) -> Option<(u64, u64)> {
    s.trim()
        .split('-')
        .map(|x| x.parse::<u64>().ok())
        .collect_tuple()
        .and_then(|(a, b)| Some((a?, b?)))
}

/// Generate all invalid numbers (pattern repeated exactly twice) within [min, max]
fn generate_invalid_part1(min: u64, max: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let max_digits = max.ilog10() as usize + 1;

    // Pattern length k produces numbers with 2k digits
    for pattern_len in 1..=max_digits / 2 {
        let pattern_min = if pattern_len == 1 {
            1
        } else {
            10u64.pow(pattern_len as u32 - 1)
        };
        let pattern_max = 10u64.pow(pattern_len as u32) - 1;

        for pattern in pattern_min..=pattern_max {
            // Construct number by repeating pattern twice
            let num = pattern * 10u64.pow(pattern_len as u32) + pattern;
            if num >= min && num <= max {
                result.push(num);
            }
        }
    }
    result
}

/// Generate all invalid numbers (pattern repeated at least twice) within [min, max]
fn generate_invalid_part2(min: u64, max: u64) -> Vec<u64> {
    let mut seen = HashSet::new();
    let max_digits = max.ilog10() as usize + 1;

    // For each pattern length
    for pattern_len in 1..=max_digits / 2 {
        let pattern_min = if pattern_len == 1 {
            1
        } else {
            10u64.pow(pattern_len as u32 - 1)
        };
        let pattern_max = 10u64.pow(pattern_len as u32) - 1;
        let multiplier = 10u64.pow(pattern_len as u32);

        // For each number of repetitions (at least 2)
        for reps in 2..=max_digits / pattern_len {
            for pattern in pattern_min..=pattern_max {
                // Build number by repeating pattern
                let mut num = 0u64;
                for _ in 0..reps {
                    num = num * multiplier + pattern;
                }

                if num >= min && num <= max {
                    seen.insert(num);
                }
            }
        }
    }

    seen.into_iter().collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges: Vec<(u64, u64)> = input.split(',').filter_map(parse_range).collect();

    let global_min = ranges.iter().map(|r| r.0).min().unwrap_or(0);
    let global_max = ranges.iter().map(|r| r.1).max().unwrap_or(0);

    let invalid_nums = generate_invalid_part1(global_min, global_max);

    let sum = invalid_nums
        .iter()
        .filter(|&&n| ranges.iter().any(|&(start, end)| n >= start && n <= end))
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges: Vec<(u64, u64)> = input.split(',').filter_map(parse_range).collect();

    let global_min = ranges.iter().map(|r| r.0).min().unwrap_or(0);
    let global_max = ranges.iter().map(|r| r.1).max().unwrap_or(0);

    let invalid_nums = generate_invalid_part2(global_min, global_max);

    let sum = invalid_nums
        .iter()
        .filter(|&&n| ranges.iter().any(|&(start, end)| n >= start && n <= end))
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
