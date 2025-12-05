advent_of_code::solution!(4);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Paper,
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '@' => Cell::Paper,
            _ => Cell::Empty,
        }
    }

    fn is_paper(self) -> bool {
        self == Cell::Paper
    }
}

/// Parse input into a grid of cells
fn parse_grid(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().map(Cell::from_char).collect())
        .collect()
}

/// Count adjacent paper rolls (8 directions) for a given position
fn count_neighbors(grid: &[Vec<Cell>], row: usize, col: usize) -> usize {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut count = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;
            if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                if grid[nr as usize][nc as usize].is_paper() {
                    count += 1;
                }
            }
        }
    }

    count
}

/// Check if a paper roll at (row, col) is accessible (fewer than 4 neighbors)
fn is_accessible(grid: &[Vec<Cell>], row: usize, col: usize) -> bool {
    grid[row][col].is_paper() && count_neighbors(grid, row, col) < 4
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_grid(input);
    let mut accessible = 0u64;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if is_accessible(&grid, row, col) {
                accessible += 1;
            }
        }
    }

    Some(accessible)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_grid(input);
    let mut total_removed = 0u64;

    loop {
        // Find all accessible rolls (fewer than 4 neighbors)
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if is_accessible(&grid, row, col) {
                    to_remove.push((row, col));
                }
            }
        }

        // If no rolls are accessible, we're done
        if to_remove.is_empty() {
            break;
        }

        // Remove all accessible rolls
        for (row, col) in &to_remove {
            grid[*row][*col] = Cell::Empty;
        }

        total_removed += to_remove.len() as u64;
    }

    Some(total_removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
