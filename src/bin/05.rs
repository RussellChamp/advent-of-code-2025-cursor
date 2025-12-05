advent_of_code::solution!(5);

/// A range of fresh ingredient IDs (inclusive)
struct FreshRange {
    start: u64,
    end: u64,
}

impl FreshRange {
    fn contains(&self, id: u64) -> bool {
        id >= self.start && id <= self.end
    }
}

/// Parse the input into fresh ranges and available IDs
fn parse_input(input: &str) -> (Vec<FreshRange>, Vec<u64>) {
    // Normalize line endings and split on blank line
    let normalized = input.replace("\r\n", "\n");
    let mut sections = normalized.split("\n\n");

    // Parse fresh ingredient ID ranges
    let ranges_section = sections.next().unwrap_or("");
    let ranges: Vec<FreshRange> = ranges_section
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split('-');
            let start: u64 = parts.next().unwrap().parse().unwrap();
            let end: u64 = parts.next().unwrap().parse().unwrap();
            FreshRange { start, end }
        })
        .collect();

    // Parse available ingredient IDs
    let ids_section = sections.next().unwrap_or("");
    let ids: Vec<u64> = ids_section
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().parse().unwrap())
        .collect();

    (ranges, ids)
}

/// Check if an ingredient ID is fresh (falls within any range)
fn is_fresh(id: u64, ranges: &[FreshRange]) -> bool {
    ranges.iter().any(|range| range.contains(id))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = parse_input(input);

    let fresh_count = ids.iter().filter(|&&id| is_fresh(id, &ranges)).count();

    Some(fresh_count as u64)
}

/// Merge overlapping ranges and count total unique IDs
fn count_unique_fresh_ids(mut ranges: Vec<FreshRange>) -> u64 {
    if ranges.is_empty() {
        return 0;
    }

    // Sort ranges by start
    ranges.sort_by_key(|r| r.start);

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
