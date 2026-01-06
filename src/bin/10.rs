advent_of_code::solution!(10);

/// Parse a machine line into (target_bitmask, buttons_bitmasks) for Part 1
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

/// Parse a machine line for Part 2: (joltage_targets, buttons as index lists)
fn parse_machine_part2(line: &str) -> Option<(Vec<u64>, Vec<Vec<usize>>)> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    // Find where joltages start and end
    let joltage_start = line.find('{')?;
    let joltage_end = line.find('}')?;
    let joltage_str = &line[joltage_start + 1..joltage_end];

    // Parse joltage targets
    let targets: Vec<u64> = joltage_str
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    // Find where buttons section ends (at joltage start)
    let bracket_end = line.find(']')?;
    let button_section = &line[bracket_end + 1..joltage_start];

    // Parse all buttons
    let mut buttons: Vec<Vec<usize>> = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = button_section.chars().collect();
    while i < chars.len() {
        if chars[i] == '(' {
            let start = i + 1;
            while i < chars.len() && chars[i] != ')' {
                i += 1;
            }
            let button_str: String = chars[start..i].iter().collect();

            let indices: Vec<usize> = button_str
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            buttons.push(indices);
        }
        i += 1;
    }

    Some((targets, buttons))
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

/// Find minimum button presses to reach target joltage levels (Part 2: addition)
/// Uses the "bifurcate" algorithm: enumerate parity-matching combos, halve, and recurse
fn min_presses_part2(targets: &[u64], buttons: &[Vec<usize>]) -> u64 {
    use std::collections::HashMap;

    let n_counters = targets.len();

    if targets.iter().all(|&t| t == 0) {
        return 0;
    }

    // Build button masks for XOR parity calculation (like Part 1)
    let button_masks: Vec<u64> = buttons
        .iter()
        .map(|button| {
            let mut mask = 0u64;
            for &idx in button {
                if idx < n_counters {
                    mask |= 1 << idx;
                }
            }
            mask
        })
        .collect();

    // Build button effects lists
    let effects: Vec<Vec<usize>> = buttons
        .iter()
        .map(|b| b.iter().filter(|&&i| i < n_counters).copied().collect())
        .collect();

    // Calculate parity mask (which bits are odd)
    fn parity_mask(targets: &[u64]) -> u64 {
        let mut mask = 0u64;
        for (i, &t) in targets.iter().enumerate() {
            if t % 2 == 1 {
                mask |= 1 << i;
            }
        }
        mask
    }

    fn solve(
        targets: Vec<u64>,
        button_masks: &[u64],
        effects: &[Vec<usize>],
        cache: &mut HashMap<Vec<u64>, u64>,
    ) -> u64 {
        // Base case: all zeros
        if targets.iter().all(|&t| t == 0) {
            return 0;
        }

        // Check cache
        if let Some(&result) = cache.get(&targets) {
            return result;
        }

        let n_buttons = button_masks.len();
        let target_parity = parity_mask(&targets);

        let mut min_presses = u64::MAX;

        // Enumerate all 2^B button combinations for Phase 1
        for combo in 0..(1u64 << n_buttons) {
            // Calculate parity this combo achieves (XOR of selected button masks)
            let mut achieved_parity = 0u64;
            let mut phase1_count = 0u64;

            for (i, &mask) in button_masks.iter().enumerate() {
                if combo & (1 << i) != 0 {
                    achieved_parity ^= mask;
                    phase1_count += 1;
                }
            }

            // Must match target parity for remaining to be all even
            if achieved_parity != target_parity {
                continue;
            }

            // Calculate remaining targets after pressing Phase 1 buttons once each
            let mut remaining = targets.clone();
            let mut valid = true;

            for (btn_idx, btn_effects) in effects.iter().enumerate() {
                if combo & (1 << btn_idx) != 0 {
                    for &counter_idx in btn_effects {
                        if remaining[counter_idx] == 0 {
                            valid = false;
                            break;
                        }
                        remaining[counter_idx] -= 1;
                    }
                    if !valid {
                        break;
                    }
                }
            }

            if !valid {
                continue;
            }

            // All remaining should now be even - halve them
            let halved: Vec<u64> = remaining.iter().map(|&r| r / 2).collect();

            // Recurse on halved problem
            let sub_result = solve(halved, button_masks, effects, cache);
            if sub_result != u64::MAX {
                // Total = Phase 1 presses + 2 * (recursive result)
                let total = phase1_count + 2 * sub_result;
                min_presses = min_presses.min(total);
            }
        }

        cache.insert(targets, min_presses);
        min_presses
    }

    let mut cache = HashMap::new();
    solve(targets.to_vec(), &button_masks, &effects, &mut cache)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0;

    for line in input.lines() {
        if let Some((targets, buttons)) = parse_machine_part2(line) {
            let presses = min_presses_part2(&targets, &buttons);
            if presses != u64::MAX {
                total += presses;
            }
        }
    }

    Some(total)
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
        assert_eq!(result, Some(33)); // 10 + 12 + 11 = 33
    }
}
