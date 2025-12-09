use std::collections::VecDeque;

advent_of_code::solution!(4);

/// Parse input into a grid of booleans (true = paper)
fn parse_grid(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

/// Get valid neighbors for a position
fn get_neighbors(row: usize, col: usize, rows: usize, cols: usize) -> impl Iterator<Item = (usize, usize)> {
    const DELTAS: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];
    
    DELTAS.into_iter().filter_map(move |(dr, dc)| {
        let nr = row as i32 + dr;
        let nc = col as i32 + dc;
        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
            Some((nr as usize, nc as usize))
        } else {
            None
        }
    })
}

/// Count adjacent paper rolls for a given position
fn count_neighbors(grid: &[Vec<bool>], row: usize, col: usize) -> u8 {
    let rows = grid.len();
    let cols = grid[0].len();
    get_neighbors(row, col, rows, cols)
        .filter(|&(nr, nc)| grid[nr][nc])
        .count() as u8
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_grid(input);
    if grid.is_empty() {
        return Some(0);
    }

    let mut accessible = 0u64;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] && count_neighbors(&grid, row, col) < 4 {
                accessible += 1;
            }
        }
    }

    Some(accessible)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_grid(input);
    if grid.is_empty() {
        return Some(0);
    }

    let rows = grid.len();
    let cols = grid[0].len();

    // Pre-compute neighbor counts for all cells
    let mut neighbor_counts: Vec<Vec<u8>> = vec![vec![0; cols]; rows];
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] {
                neighbor_counts[row][col] = count_neighbors(&grid, row, col);
            }
        }
    }

    // Initialize queue with all accessible cells (paper with < 4 neighbors)
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut in_queue: Vec<Vec<bool>> = vec![vec![false; cols]; rows];

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] && neighbor_counts[row][col] < 4 {
                queue.push_back((row, col));
                in_queue[row][col] = true;
            }
        }
    }

    let mut total_removed = 0u64;

    while let Some((row, col)) = queue.pop_front() {
        in_queue[row][col] = false;

        // Skip if already removed or no longer accessible
        if !grid[row][col] || neighbor_counts[row][col] >= 4 {
            continue;
        }

        // Remove this cell
        grid[row][col] = false;
        total_removed += 1;

        // Update neighbor counts for adjacent cells and potentially add them to queue
        for (nr, nc) in get_neighbors(row, col, rows, cols) {
            if grid[nr][nc] {
                neighbor_counts[nr][nc] = neighbor_counts[nr][nc].saturating_sub(1);
                // If this cell became accessible and isn't already queued, add it
                if neighbor_counts[nr][nc] < 4 && !in_queue[nr][nc] {
                    queue.push_back((nr, nc));
                    in_queue[nr][nc] = true;
                }
            }
        }
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
