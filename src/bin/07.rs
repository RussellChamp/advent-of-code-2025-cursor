use std::collections::{HashMap, HashSet};

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
/// Beams merge when they occupy the same column (HashSet)
fn count_splits(input: &str) -> u64 {
    let (grid, start_col) = parse_grid(input);
    if grid.is_empty() {
        return 0;
    }

    let width = grid[0].len();
    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(start_col);

    let mut split_count: u64 = 0;

    for row in grid.iter().skip(1) {
        let mut new_beams: HashSet<usize> = HashSet::new();

        for &col in &beams {
            match row[col] {
                '^' => {
                    split_count += 1;
                    if col > 0 {
                        new_beams.insert(col - 1);
                    }
                    if col + 1 < width {
                        new_beams.insert(col + 1);
                    }
                }
                _ => {
                    new_beams.insert(col);
                }
            }
        }

        beams = new_beams;
        if beams.is_empty() {
            break;
        }
    }

    split_count
}

/// Count the number of unique timelines (paths) through the manifold
/// Each particle at a position represents independent timelines - they don't merge
fn count_timelines(input: &str) -> u64 {
    let (grid, start_col) = parse_grid(input);
    if grid.is_empty() {
        return 0;
    }

    let width = grid[0].len();

    // Track number of particles (timelines) at each column position
    let mut particles: HashMap<usize, u64> = HashMap::new();
    particles.insert(start_col, 1);

    for row in grid.iter().skip(1) {
        let mut new_particles: HashMap<usize, u64> = HashMap::new();

        for (&col, &count) in &particles {
            match row[col] {
                '^' => {
                    // Each particle splits into 2 timelines (left and right)
                    if col > 0 {
                        *new_particles.entry(col - 1).or_insert(0) += count;
                    }
                    if col + 1 < width {
                        *new_particles.entry(col + 1).or_insert(0) += count;
                    }
                }
                _ => {
                    // Particle continues straight down
                    *new_particles.entry(col).or_insert(0) += count;
                }
            }
        }

        particles = new_particles;
        if particles.is_empty() {
            break;
        }
    }

    // Sum all particles - each represents a unique timeline
    particles.values().sum()
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
