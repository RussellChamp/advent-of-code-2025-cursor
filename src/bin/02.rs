advent_of_code::solution!(2);

/// Check if a number is "invalid" - made of a pattern repeated exactly twice
fn is_invalid_part1(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    // Must have even length for pattern to repeat exactly twice
    if len % 2 != 0 {
        return false;
    }
    let half = len / 2;
    s[..half] == s[half..]
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0u64;

    // Parse comma-separated ranges, handling potential whitespace/newlines
    for part in input.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        // Parse "start-end" range
        let mut range_parts = part.split('-');
        let start: u64 = range_parts.next()?.parse().ok()?;
        let end: u64 = range_parts.next()?.parse().ok()?;

        // Check each number in the range
        for n in start..=end {
            if is_invalid_part1(n) {
                sum += n;
            }
        }
    }

    Some(sum)
}

/// Check if a number is "invalid" - made of a pattern repeated at least twice
fn is_invalid_part2(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Try each possible pattern length (1 to len/2)
    for pattern_len in 1..=len / 2 {
        // Pattern can only work if it divides evenly
        if len % pattern_len != 0 {
            continue;
        }

        let pattern = &s[..pattern_len];
        let mut is_match = true;

        // Check if the entire string is made of this pattern repeated
        for i in (pattern_len..len).step_by(pattern_len) {
            if &s[i..i + pattern_len] != pattern {
                is_match = false;
                break;
            }
        }

        if is_match {
            return true;
        }
    }

    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0u64;

    for part in input.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        let mut range_parts = part.split('-');
        let start: u64 = range_parts.next()?.parse().ok()?;
        let end: u64 = range_parts.next()?.parse().ok()?;

        for n in start..=end {
            if is_invalid_part2(n) {
                sum += n;
            }
        }
    }

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
