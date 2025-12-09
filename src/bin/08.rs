advent_of_code::solution!(8);

// Helper function to parse a line like "162,817,812" into (x, y, z)
fn parse_line(line: &str) -> Option<(i64, i64, i64)> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 3 {
        return None;
    }

    let x = parts[0].parse::<i64>().ok()?;
    let y = parts[1].parse::<i64>().ok()?;
    let z = parts[2].parse::<i64>().ok()?;

    Some((x, y, z))
}

// Calculate distance squared between two points (we don't need actual distance for comparing)
fn get_distance(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> i64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    let dz = p1.2 - p2.2;
    dx * dx + dy * dy + dz * dz
}

pub fn part_one(input: &str) -> Option<u64> {
    // Step 1: Parse all the coordinates
    let mut coords: Vec<(i64, i64, i64)> = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(coord) = parse_line(line) {
            coords.push(coord);
        }
    }

    let n = coords.len();
    if n == 0 {
        return None;
    }

    // Step 2: Calculate all pair distances
    // Store as (distance, index1, index2)
    let mut all_pairs: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            let dist = get_distance(coords[i], coords[j]);
            all_pairs.push((dist, i, j));
        }
    }

    // Step 3: Sort by distance (smallest first)
    all_pairs.sort_by(|a, b| a.0.cmp(&b.0));

    // Step 4: Track which group each junction box belongs to
    // Initially each box is in its own group (group number = its index)
    let mut groups: Vec<usize> = (0..n).collect();

    // Step 5: Connect the first 1000 pairs
    let connections_to_make = 1000;
    let mut connections_made = 0;

    for (_, i, j) in &all_pairs {
        if connections_made >= connections_to_make {
            break;
        }

        // Merge the groups - put everyone from group j into group i
        let group_i = groups[*i];
        let group_j = groups[*j];

        if group_i != group_j {
            // Merge: change all boxes in group_j to be in group_i
            for k in 0..n {
                if groups[k] == group_j {
                    groups[k] = group_i;
                }
            }
        }

        connections_made += 1;
    }

    // Step 6: Count how many boxes are in each group
    let mut group_sizes: Vec<usize> = Vec::new();
    let mut counted_groups: Vec<usize> = Vec::new();

    for i in 0..n {
        let my_group = groups[i];
        if !counted_groups.contains(&my_group) {
            // Count how many are in this group
            let mut count = 0;
            for j in 0..n {
                if groups[j] == my_group {
                    count += 1;
                }
            }
            group_sizes.push(count);
            counted_groups.push(my_group);
        }
    }

    // Step 7: Sort sizes descending and multiply top 3
    group_sizes.sort_by(|a, b| b.cmp(a));

    let mut result: u64 = 1;
    for i in 0..3 {
        if i < group_sizes.len() {
            result *= group_sizes[i] as u64;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Step 1: Parse all the coordinates
    let mut coords: Vec<(i64, i64, i64)> = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(coord) = parse_line(line) {
            coords.push(coord);
        }
    }

    let n = coords.len();
    if n <= 1 {
        return None;
    }

    // Step 2: Calculate all pair distances
    let mut all_pairs: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            let dist = get_distance(coords[i], coords[j]);
            all_pairs.push((dist, i, j));
        }
    }

    // Step 3: Sort by distance
    all_pairs.sort_by(|a, b| a.0.cmp(&b.0));

    // Step 4: Track groups
    let mut groups: Vec<usize> = (0..n).collect();

    // Step 5: Keep connecting until everyone is in one group
    let mut last_i = 0;
    let mut last_j = 0;

    for (_, i, j) in &all_pairs {
        let group_i = groups[*i];
        let group_j = groups[*j];

        if group_i != group_j {
            // This connection actually merges two different groups
            last_i = *i;
            last_j = *j;

            // Merge the groups
            for k in 0..n {
                if groups[k] == group_j {
                    groups[k] = group_i;
                }
            }

            // Check if everyone is now in the same group
            let first_group = groups[0];
            let mut all_same = true;
            for k in 1..n {
                if groups[k] != first_group {
                    all_same = false;
                    break;
                }
            }

            if all_same {
                break;
            }
        }
    }

    // Step 6: Return X coordinates multiplied
    let x1 = coords[last_i].0 as u64;
    let x2 = coords[last_j].0 as u64;

    Some(x1 * x2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // The example uses 10 connections, not 1000
        // So we need to test the logic differently
        let input = &advent_of_code::template::read_file("examples", DAY);

        // Parse coordinates
        let mut coords: Vec<(i64, i64, i64)> = Vec::new();
        for line in input.lines() {
            let line = line.trim();
            if !line.is_empty() {
                if let Some(coord) = parse_line(line) {
                    coords.push(coord);
                }
            }
        }

        let n = coords.len();

        // Get all pairs sorted
        let mut all_pairs: Vec<(i64, usize, usize)> = Vec::new();
        for i in 0..n {
            for j in i + 1..n {
                let dist = get_distance(coords[i], coords[j]);
                all_pairs.push((dist, i, j));
            }
        }
        all_pairs.sort_by(|a, b| a.0.cmp(&b.0));

        // Connect 10 pairs
        let mut groups: Vec<usize> = (0..n).collect();
        for k in 0..10 {
            let (_, i, j) = all_pairs[k];
            let group_i = groups[i];
            let group_j = groups[j];
            if group_i != group_j {
                for m in 0..n {
                    if groups[m] == group_j {
                        groups[m] = group_i;
                    }
                }
            }
        }

        // Count group sizes
        let mut group_sizes: Vec<usize> = Vec::new();
        let mut counted: Vec<usize> = Vec::new();
        for i in 0..n {
            if !counted.contains(&groups[i]) {
                let count = groups.iter().filter(|&&g| g == groups[i]).count();
                group_sizes.push(count);
                counted.push(groups[i]);
            }
        }

        group_sizes.sort_by(|a, b| b.cmp(a));
        let result: u64 = group_sizes.iter().take(3).map(|&s| s as u64).product();

        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}

