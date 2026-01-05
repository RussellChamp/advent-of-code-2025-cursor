advent_of_code::solution!(9);

use itertools::Itertools;
use std::collections::HashMap;

fn parse_coords(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let (x, y) = line.split_once(',')?;
            Some((x.trim().parse().ok()?, y.trim().parse().ok()?))
        })
        .collect()
}

fn rectangle_area(p1: (i64, i64), p2: (i64, i64)) -> u64 {
    let width = (p2.0 - p1.0).abs() + 1;
    let height = (p2.1 - p1.1).abs() + 1;
    (width * height) as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let coords = parse_coords(input);

    coords
        .iter()
        .tuple_combinations()
        .map(|(&p1, &p2)| rectangle_area(p1, p2))
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    let coords = parse_coords(input);
    if coords.len() < 3 {
        return None;
    }

    // Step 1: Coordinate compression
    let mut xs: Vec<i64> = coords.iter().map(|p| p.0).collect();
    let mut ys: Vec<i64> = coords.iter().map(|p| p.1).collect();
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();

    let x_to_idx: HashMap<i64, usize> = xs.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    let y_to_idx: HashMap<i64, usize> = ys.iter().enumerate().map(|(i, &y)| (y, i)).collect();

    let nx = xs.len();
    let ny = ys.len();

    // Step 2: Create expanded grid (2x+1 to handle gaps between coordinates)
    // Even indices = actual coordinate lines, odd indices = gaps between them
    let gx = 2 * nx + 1;
    let gy = 2 * ny + 1;

    // Step 3: Mark boundary edges in expanded grid
    let mut is_boundary = vec![vec![false; gy]; gx];

    for i in 0..coords.len() {
        let (x1, y1) = coords[i];
        let (x2, y2) = coords[(i + 1) % coords.len()];

        // Convert to expanded grid coords (actual coords at even indices, +1 offset)
        let gx1 = 2 * x_to_idx[&x1] + 1;
        let gy1 = 2 * y_to_idx[&y1] + 1;
        let gx2 = 2 * x_to_idx[&x2] + 1;
        let gy2 = 2 * y_to_idx[&y2] + 1;

        // Draw line in expanded grid
        if gx1 == gx2 {
            // Vertical line
            let (y_min, y_max) = if gy1 < gy2 { (gy1, gy2) } else { (gy2, gy1) };
            for y in y_min..=y_max {
                is_boundary[gx1][y] = true;
            }
        } else {
            // Horizontal line
            let (x_min, x_max) = if gx1 < gx2 { (gx1, gx2) } else { (gx2, gx1) };
            for x in x_min..=x_max {
                is_boundary[x][gy1] = true;
            }
        }
    }

    // Step 4: Flood fill from outside (corner 0,0 is always outside)
    let mut outside = vec![vec![false; gy]; gx];
    let mut stack = vec![(0usize, 0usize)];

    while let Some((x, y)) = stack.pop() {
        if x >= gx || y >= gy || is_boundary[x][y] || outside[x][y] {
            continue;
        }
        outside[x][y] = true;
        if x > 0 {
            stack.push((x - 1, y));
        }
        stack.push((x + 1, y));
        if y > 0 {
            stack.push((x, y - 1));
        }
        stack.push((x, y + 1));
    }

    // Step 5: Build 2D prefix sum counting OUTSIDE cells
    let mut prefix = vec![vec![0i64; gy + 1]; gx + 1];
    for i in 0..gx {
        for j in 0..gy {
            prefix[i + 1][j + 1] = prefix[i + 1][j] + prefix[i][j + 1] - prefix[i][j]
                + if outside[i][j] { 1 } else { 0 };
        }
    }

    // Helper to count outside cells in expanded rectangle [i1, i2] x [j1, j2]
    let count_outside = |i1: usize, i2: usize, j1: usize, j2: usize| -> i64 {
        prefix[i2 + 1][j2 + 1] - prefix[i1][j2 + 1] - prefix[i2 + 1][j1] + prefix[i1][j1]
    };

    // Step 6: Check all pairs of red tiles
    let mut max_area = 0u64;

    for (idx1, &(x1, y1)) in coords.iter().enumerate() {
        for &(x2, y2) in coords.iter().skip(idx1 + 1) {
            // Convert to expanded grid coords
            let gi1 = 2 * x_to_idx[&x1] + 1;
            let gj1 = 2 * y_to_idx[&y1] + 1;
            let gi2 = 2 * x_to_idx[&x2] + 1;
            let gj2 = 2 * y_to_idx[&y2] + 1;

            let (i1, i2) = if gi1 < gi2 { (gi1, gi2) } else { (gi2, gi1) };
            let (j1, j2) = if gj1 < gj2 { (gj1, gj2) } else { (gj2, gj1) };

            // Rectangle is valid if it contains no outside cells
            if count_outside(i1, i2, j1, j2) == 0 {
                let area = rectangle_area((x1, y1), (x2, y2));
                max_area = max_area.max(area);
            }
        }
    }

    Some(max_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
