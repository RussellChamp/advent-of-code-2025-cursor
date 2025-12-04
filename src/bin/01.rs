advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut position: i32 = 50;
    let mut count = 0u64;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = &line[0..1];
        let distance: i32 = line[1..].parse().ok()?;

        match direction {
            "L" => position = (position - distance).rem_euclid(100),
            "R" => position = (position + distance).rem_euclid(100),
            _ => return None,
        }

        if position == 0 {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut position: i32 = 50;
    let mut count = 0u64;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = &line[0..1];
        let distance: i32 = line[1..].parse().ok()?;

        // Count how many times we pass through 0 during this rotation
        let zeros = match direction {
            "L" => {
                // Moving left: we hit 0 at clicks p, p+100, p+200, ... (if p > 0)
                // or at clicks 100, 200, 300, ... (if p = 0)
                if position == 0 {
                    distance / 100
                } else if distance >= position {
                    (distance - position) / 100 + 1
                } else {
                    0
                }
            }
            "R" => {
                // Moving right: we hit 0 at click (100-p), then (100-p)+100, etc.
                // or at clicks 100, 200, 300, ... (if p = 0)
                if position == 0 {
                    distance / 100
                } else if distance >= 100 - position {
                    (distance + position - 100) / 100 + 1
                } else {
                    0
                }
            }
            _ => return None,
        };

        count += zeros as u64;

        // Update position
        match direction {
            "L" => position = (position - distance).rem_euclid(100),
            "R" => position = (position + distance).rem_euclid(100),
            _ => {}
        }
    }

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
