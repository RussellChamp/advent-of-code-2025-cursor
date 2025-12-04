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

/// Check if a number is "invalid" - made of a pattern repeated exactly twice
fn is_invalid_part1(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    // Must have even length for pattern to repeat exactly twice
    len % 2 == 0 && s[..len / 2] == s[len / 2..]
}

/// Check if a number is "invalid" - made of a pattern repeated at least twice
fn is_invalid_part2(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Try each possible pattern length (1 to len/2)
    (1..=len / 2).any(|pattern_len| {
        // Pattern can only work if it divides evenly
        len % pattern_len == 0 && {
            let pattern = &s[..pattern_len];
            // Check if all chunks match the pattern
            s.as_bytes()
                .chunks(pattern_len)
                .all(|chunk| chunk == pattern.as_bytes())
        }
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let sum = input
        .split(',')
        .filter_map(|part| parse_range(part))
        .flat_map(|(start, end)| start..=end)
        .filter(|&n| is_invalid_part1(n))
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum = input
        .split(',')
        .filter_map(|part| parse_range(part))
        .flat_map(|(start, end)| start..=end)
        .filter(|&n| is_invalid_part2(n))
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
