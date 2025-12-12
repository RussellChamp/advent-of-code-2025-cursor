advent_of_code::solution!(10);

/// Parse a machine line into (target_bitmask, buttons_bitmasks)
fn parse_machine(line: &str) -> Option<(u64, Vec<u64>)> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    // Find the pattern between []
    let bracket_start = line.find('[')?;
    let bracket_end = line.find(']')?;
    let pattern = &line[bracket_start + 1..bracket_end];

    // Convert pattern to target bitmask (# = on = 1, . = off = 0)
    let mut target: u64 = 0;
    for (i, c) in pattern.chars().enumerate() {
        if c == '#' {
            target |= 1 << i;
        }
    }

    // Find where joltages start (curly brace)
    let joltage_start = line.find('{')?;

    // Parse all buttons between ] and {
    let button_section = &line[bracket_end + 1..joltage_start];
    let mut buttons = Vec::new();

    let mut i = 0;
    let chars: Vec<char> = button_section.chars().collect();
    while i < chars.len() {
        if chars[i] == '(' {
            // Find matching )
            let start = i + 1;
            while i < chars.len() && chars[i] != ')' {
                i += 1;
            }
            let button_str: String = chars[start..i].iter().collect();

            // Parse the indices
            let mut button_mask: u64 = 0;
            for num_str in button_str.split(',') {
                if let Ok(idx) = num_str.trim().parse::<usize>() {
                    button_mask |= 1 << idx;
                }
            }
            buttons.push(button_mask);
        }
        i += 1;
    }

    Some((target, buttons))
}

/// Find minimum button presses to reach target state (Part 1: toggle/XOR)
fn min_presses_part1(target: u64, buttons: &[u64]) -> u64 {
    let n = buttons.len();
    let mut min_presses = u64::MAX;

    // Try all 2^n combinations of buttons (each pressed 0 or 1 times)
    for mask in 0..(1u64 << n) {
        let mut result: u64 = 0;
        let mut presses = 0;

        for (i, &button) in buttons.iter().enumerate() {
            if mask & (1 << i) != 0 {
                result ^= button;
                presses += 1;
            }
        }

        if result == target && presses < min_presses {
            min_presses = presses;
        }
    }

    min_presses
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;

    for line in input.lines() {
        if let Some((target, buttons)) = parse_machine(line) {
            let presses = min_presses_part1(target, &buttons);
            if presses != u64::MAX {
                total += presses;
            }
        }
    }

    Some(total)
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
        assert_eq!(result, Some(7)); // 2 + 3 + 2 = 7
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
