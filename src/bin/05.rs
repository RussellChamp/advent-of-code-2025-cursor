advent_of_code::solution!(5);

/// A range of fresh ingredient IDs (inclusive)
#[derive(Clone)]
struct FreshRange {
    start: u64,
    end: u64,
}

/// Parse the input into fresh ranges and available IDs without extra allocations
fn parse_input(input: &str) -> (Vec<FreshRange>, Vec<u64>) {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    let mut in_ids_section = false;

    for line in input.lines() {
        let line = line.trim().trim_end_matches('\r');
        if line.is_empty() {
            in_ids_section = true;
            continue;
        }

        if in_ids_section {
            if let Ok(id) = line.parse() {
                ids.push(id);
            }
        } else if let Some((start_str, end_str)) = line.split_once('-') {
            if let (Ok(start), Ok(end)) = (start_str.parse(), end_str.parse()) {
                ranges.push(FreshRange { start, end });
            }
        }
    }

    (ranges, ids)
}

/// Check if an ingredient ID is fresh using binary search on sorted ranges
fn is_fresh_binary(id: u64, sorted_ranges: &[FreshRange]) -> bool {
    // Binary search to find a range that might contain id
    // Find the rightmost range where start <= id
    let idx = sorted_ranges.partition_point(|r| r.start <= id);
    if idx == 0 {
        return false;
    }
    // Check if the range at idx-1 contains id
    sorted_ranges[idx - 1].end >= id
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut ranges, ids) = parse_input(input);

    // Sort ranges by start for binary search
    ranges.sort_unstable_by_key(|r| r.start);

    let fresh_count = ids
        .iter()
        .filter(|&&id| is_fresh_binary(id, &ranges))
        .count();

    Some(fresh_count as u64)
}

/// Merge overlapping ranges and count total unique IDs
fn count_unique_fresh_ids(mut ranges: Vec<FreshRange>) -> u64 {
    if ranges.is_empty() {
        return 0;
    }

    // Sort ranges by start
    ranges.sort_unstable_by_key(|r| r.start);

    // Merge overlapping ranges
    let mut merged: Vec<FreshRange> = Vec::new();
    merged.push(FreshRange {
        start: ranges[0].start,
        end: ranges[0].end,
    });

    for range in ranges.into_iter().skip(1) {
        let last = merged.last_mut().unwrap();
        // Check if ranges overlap or are adjacent
        if range.start <= last.end + 1 {
            // Extend the current range if needed
            last.end = last.end.max(range.end);
        } else {
            // No overlap, start a new range
            merged.push(range);
        }
    }

    // Count total unique IDs across all merged ranges
    merged.iter().map(|r| r.end - r.start + 1).sum()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = parse_input(input);
    Some(count_unique_fresh_ids(ranges))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
