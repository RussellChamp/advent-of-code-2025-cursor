advent_of_code::solution!(3);

/// Find the maximum 2-digit number by selecting exactly 2 batteries from a bank
fn max_joltage_2(bank: &[u8]) -> u64 {
    let n = bank.len();

    // Precompute suffix maximums: suffix_max[i] = max digit from position i to end
    // This avoids O(n) scan for each position, making overall O(n) instead of O(n²)
    let mut suffix_max = vec![0u8; n];
    suffix_max[n - 1] = bank[n - 1];
    for i in (0..n - 1).rev() {
        suffix_max[i] = bank[i].max(suffix_max[i + 1]);
    }

    let mut max = 0u64;
    for i in 0..n - 1 {
        let tens = bank[i] as u64;
        let units = suffix_max[i + 1] as u64;  // O(1) lookup instead of O(n) scan
        let joltage = tens * 10 + units;
        max = max.max(joltage);
    }

    max
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0u64;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Convert line to digits
        let digits: Vec<u8> = line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        total += max_joltage_2(&digits);
    }

    Some(total)
}

/// Find the maximum k-digit number by selecting exactly k batteries from a bank
fn max_joltage_k(bank: &[u8], k: usize) -> u64 {
    let n = bank.len();
    let mut result = 0u64;
    let mut start = 0;

    // Greedily pick each of the k digits
    for remaining in (1..=k).rev() {
        // We can pick from [start, n - remaining] inclusive
        let end = n - remaining;

        // Find the FIRST position of the max digit in this range
        let mut best_pos = 0;
        let mut best_digit = bank[start];
        for (i, &d) in bank[start..=end].iter().enumerate() {
            if d > best_digit {
                best_digit = d;
                best_pos = i;
            }
        }

        result = result * 10 + best_digit as u64;
        start = start + best_pos + 1;
    }

    result
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0u64;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let digits: Vec<u8> = line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        total += max_joltage_k(&digits, 12);
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
