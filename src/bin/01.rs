advent_of_code::solution!(1);

/// Parse a rotation instruction like "L68" or "R14" into (direction, distance)
fn parse_rotation(line: &str) -> Option<(char, i32)> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    let direction = line.chars().next()?;
    let distance: i32 = line[1..].parse().ok()?;
    Some((direction, distance))
}

/// Apply a rotation and return the new position
fn apply_rotation(position: i32, direction: char, distance: i32) -> i32 {
    match direction {
        'L' => (position - distance).rem_euclid(100),
        'R' => (position + distance).rem_euclid(100),
        _ => position,
    }
}

/// Count how many times we pass through 0 during a rotation
fn count_zeros(position: i32, direction: char, distance: i32) -> i32 {
    match direction {
        'L' => {
            if position == 0 {
                distance / 100
            } else if distance >= position {
                (distance - position) / 100 + 1
            } else {
                0
            }
        }
        'R' => {
            if position == 0 {
                distance / 100
            } else if distance >= 100 - position {
                (distance + position - 100) / 100 + 1
            } else {
                0
            }
        }
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let count = input
        .lines()
        .filter_map(parse_rotation)
        .fold((50i32, 0u64), |(pos, count), (dir, dist)| {
            let new_pos = apply_rotation(pos, dir, dist);
            let new_count = count + if new_pos == 0 { 1 } else { 0 };
            (new_pos, new_count)
        })
        .1;

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let count = input
        .lines()
        .filter_map(parse_rotation)
        .fold((50i32, 0u64), |(pos, count), (dir, dist)| {
            let zeros = count_zeros(pos, dir, dist) as u64;
            let new_pos = apply_rotation(pos, dir, dist);
            (new_pos, count + zeros)
        })
        .1;

    Some(count)
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
        assert_eq!(result, Some(6));
    }
}
