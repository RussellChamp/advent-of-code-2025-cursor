advent_of_code::solution!(7);

/// Parse the grid and find the starting column
fn parse_grid(input: &str) -> (Vec<Vec<char>>, usize) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let start_col = grid[0]
        .iter()
        .position(|&c| c == 'S')
        .expect("No starting position 'S' found");

    (grid, start_col)
}

/// Simulate tachyon beams through the manifold and count splits
/// Uses Vec<bool> instead of HashSet for beam positions
fn count_splits(input: &str) -> u64 {
    let (grid, start_col) = parse_grid(input);
    if grid.is_empty() {
        return 0;
    }

    let width = grid[0].len();
    let mut beams = vec![false; width];
    let mut new_beams = vec![false; width];
    beams[start_col] = true;

    let mut split_count: u64 = 0;

    for row in grid.iter().skip(1) {
        new_beams.fill(false);
        let mut has_beams = false;

        for col in 0..width {
            if !beams[col] {
                continue;
            }
            match row[col] {
                '^' => {
                    split_count += 1;
                    if col > 0 {
                        new_beams[col - 1] = true;
                        has_beams = true;
                    }
                    if col + 1 < width {
                        new_beams[col + 1] = true;
                        has_beams = true;
                    }
                }
                _ => {
                    new_beams[col] = true;
                    has_beams = true;
                }
            }
        }

        std::mem::swap(&mut beams, &mut new_beams);
        if !has_beams {
            break;
        }
    }

    split_count
}

/// Count the number of unique timelines (paths) through the manifold
/// Uses Vec<u64> instead of HashMap for particle counts
fn count_timelines(input: &str) -> u64 {
    let (grid, start_col) = parse_grid(input);
    if grid.is_empty() {
        return 0;
    }

    let width = grid[0].len();
    let mut particles = vec![0u64; width];
    let mut new_particles = vec![0u64; width];
    particles[start_col] = 1;

    for row in grid.iter().skip(1) {
        new_particles.fill(0);

        for col in 0..width {
            let count = particles[col];
            if count == 0 {
                continue;
            }
            match row[col] {
                '^' => {
                    // Each particle splits into 2 timelines (left and right)
                    if col > 0 {
                        new_particles[col - 1] += count;
                    }
                    if col + 1 < width {
                        new_particles[col + 1] += count;
                    }
                }
                _ => {
                    // Particle continues straight down
                    new_particles[col] += count;
                }
            }
        }

        std::mem::swap(&mut particles, &mut new_particles);
        if particles.iter().all(|&c| c == 0) {
            break;
        }
    }

    particles.iter().sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(count_splits(input))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(count_timelines(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
