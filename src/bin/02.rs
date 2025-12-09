use itertools::Itertools;

advent_of_code::solution!(2);

/// Parse a range string like "11-22" into (start, end)
fn parse_range(s: &str) -> Option<(u64, u64)> {
    s.trim()
        .split('-')
        .map(|x| x.parse::<u64>().ok())
        .collect_tuple()
        .and_then(|(a, b)| Some((a?, b?)))
}

/// Iterator over invalid numbers (pattern repeated exactly twice) within [min, max]
fn iter_invalid_part1(min: u64, max: u64) -> impl Iterator<Item = u64> {
    let max_digits = if max == 0 { 1 } else { max.ilog10() as usize + 1 };

    (1..=max_digits / 2).flat_map(move |pattern_len| {
        let pattern_min = if pattern_len == 1 {
            1
        } else {
            10u64.pow(pattern_len as u32 - 1)
        };
        let pattern_max = 10u64.pow(pattern_len as u32) - 1;
        let multiplier = 10u64.pow(pattern_len as u32);

        (pattern_min..=pattern_max).filter_map(move |pattern| {
            let num = pattern * multiplier + pattern;
            if num >= min && num <= max {
                Some(num)
            } else {
                None
            }
        })
    })
}

/// Iterator over invalid numbers (pattern repeated at least twice) within [min, max]
fn iter_invalid_part2(min: u64, max: u64) -> impl Iterator<Item = u64> {
    let max_digits = if max == 0 { 1 } else { max.ilog10() as usize + 1 };

    (1..=max_digits / 2).flat_map(move |pattern_len| {
        let pattern_min = if pattern_len == 1 {
            1
        } else {
            10u64.pow(pattern_len as u32 - 1)
        };
        let pattern_max = 10u64.pow(pattern_len as u32) - 1;
        let multiplier = 10u64.pow(pattern_len as u32);

        (2..=max_digits / pattern_len).flat_map(move |reps| {
            (pattern_min..=pattern_max).filter_map(move |pattern| {
                let mut num = 0u64;
                for _ in 0..reps {
                    num = num * multiplier + pattern;
                }
                if num >= min && num <= max {
                    Some(num)
                } else {
                    None
                }
            })
        })
    })
}

/// Merge overlapping ranges to avoid duplicate counting
fn merge_ranges(ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return Vec::new();
    }

    let mut sorted: Vec<_> = ranges.to_vec();
    sorted.sort_by_key(|r| r.0);

    let mut merged = Vec::with_capacity(sorted.len());
    merged.push(sorted[0]);

    for &(start, end) in &sorted[1..] {
        let last = merged.last_mut().unwrap();
        if start <= last.1 + 1 {
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    }

    merged
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges: Vec<(u64, u64)> = input.split(',').filter_map(parse_range).collect();
    let merged = merge_ranges(&ranges);

    // Sum invalid numbers directly from iterator, checking merged ranges
    let sum: u64 = merged
        .iter()
        .flat_map(|&(start, end)| iter_invalid_part1(start, end))
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges: Vec<(u64, u64)> = input.split(',').filter_map(parse_range).collect();
    let merged = merge_ranges(&ranges);

    // Use itertools::unique() for deduplication
    // (needed because same number can match multiple pattern lengths)
    let sum: u64 = merged
        .iter()
        .flat_map(|&(start, end)| iter_invalid_part2(start, end))
        .unique()
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
