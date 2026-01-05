advent_of_code::solution!(12);

use std::collections::HashSet;
use dlx::choose::{mrv_chooser, no_tiebreak, prefer_any};
use dlx::x::{INodes, ONodes, Problem};
use dlx::{OptOrder, Solver, Uint};

/// Represents a shape as a set of (row, col) offsets from the top-left corner
type Shape = Vec<(i32, i32)>;

/// Parse a shape from its visual representation
fn parse_shape(lines: &[&str]) -> Shape {
    let mut cells = Vec::new();
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                cells.push((row as i32, col as i32));
            }
        }
    }
    cells
}

/// Normalize a shape so its minimum row and col are 0
fn normalize(shape: &Shape) -> Shape {
    if shape.is_empty() {
        return shape.clone();
    }
    let min_row = shape.iter().map(|&(r, _)| r).min().unwrap();
    let min_col = shape.iter().map(|&(_, c)| c).min().unwrap();
    shape.iter().map(|&(r, c)| (r - min_row, c - min_col)).collect()
}

/// Rotate a shape 90 degrees clockwise
fn rotate(shape: &Shape) -> Shape {
    // (r, c) -> (c, -r)
    normalize(&shape.iter().map(|&(r, c)| (c, -r)).collect())
}

/// Flip a shape horizontally
fn flip(shape: &Shape) -> Shape {
    // (r, c) -> (r, -c)
    normalize(&shape.iter().map(|&(r, c)| (r, -c)).collect())
}

/// Get all unique orientations (rotations and flips) of a shape
fn all_orientations(shape: &Shape) -> Vec<Shape> {
    let mut orientations: Vec<Shape> = Vec::new();
    let mut seen: HashSet<Vec<(i32, i32)>> = HashSet::new();

    let mut current = normalize(shape);

    // Try all 4 rotations
    for _ in 0..4 {
        let mut sorted = current.clone();
        sorted.sort();
        if seen.insert(sorted) {
            orientations.push(current.clone());
        }
        current = rotate(&current);
    }

    // Flip and try all 4 rotations
    current = flip(shape);
    for _ in 0..4 {
        let mut sorted = current.clone();
        sorted.sort();
        if seen.insert(sorted) {
            orientations.push(current.clone());
        }
        current = rotate(&current);
    }

    orientations
}

/// Parse the input into shapes and regions
fn parse_input(input: &str) -> (Vec<Vec<Shape>>, Vec<(usize, usize, Vec<usize>)>) {
    let mut shapes: Vec<Vec<Shape>> = Vec::new();
    let mut regions: Vec<(usize, usize, Vec<usize>)> = Vec::new();

    let mut current_shape_lines: Vec<&str> = Vec::new();
    let mut in_shape = false;

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            // Finalize current shape if any
            if !current_shape_lines.is_empty() {
                let base_shape = parse_shape(&current_shape_lines);
                shapes.push(all_orientations(&base_shape));
                current_shape_lines.clear();
            }
            in_shape = false;
            continue;
        }

        // Check if this is a shape header (e.g., "0:", "1:", etc.)
        if line.ends_with(':') && line.len() <= 3 && line.chars().next().unwrap().is_ascii_digit() {
            // Finalize previous shape if any
            if !current_shape_lines.is_empty() {
                let base_shape = parse_shape(&current_shape_lines);
                shapes.push(all_orientations(&base_shape));
                current_shape_lines.clear();
            }
            in_shape = true;
            continue;
        }

        // Check if this is a region line (e.g., "38x36: 46 32 39 35 27 29")
        if line.contains('x') && line.contains(':') {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 && parts[0].contains('x') {
                let dims: Vec<usize> = parts[0]
                    .split('x')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();

                if dims.len() == 2 {
                    let quantities: Vec<usize> = parts[1]
                        .split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();

                    regions.push((dims[0], dims[1], quantities));
                    continue;
                }
            }
        }

        // Otherwise, it's part of a shape definition
        if in_shape {
            current_shape_lines.push(line);
        }
    }

    // Finalize last shape if any
    if !current_shape_lines.is_empty() {
        let base_shape = parse_shape(&current_shape_lines);
        shapes.push(all_orientations(&base_shape));
    }

    (shapes, regions)
}

/// Solve using Dancing Links (DLX) algorithm
/// Returns true if all pieces can be placed in the grid
fn can_fit_dlx(shapes: &[Vec<Shape>], width: usize, height: usize, quantities: &[usize]) -> bool {
    // Quick area check first
    let total_cells: usize = quantities
        .iter()
        .enumerate()
        .map(|(i, &qty)| {
            if i < shapes.len() && !shapes[i].is_empty() {
                qty * shapes[i][0].len()
            } else {
                0
            }
        })
        .sum();

    let area = width * height;
    if total_cells > area {
        return false;
    }

    // Count total pieces to place
    let total_pieces: usize = quantities.iter().sum();
    if total_pieces == 0 {
        return true;
    }

    // DLX formulation:
    // - Primary items (0..total_pieces): one for each piece copy we must place
    // - Secondary items (total_pieces..total_pieces+area): grid cells (can be covered 0 or 1 times)
    let num_primary = total_pieces;
    let num_secondary = area;
    let num_items = num_primary + num_secondary;

    // Build piece copy indices for each shape type
    // piece_starts[i] = starting index in primary items for shape i
    let mut piece_starts: Vec<usize> = Vec::with_capacity(quantities.len() + 1);
    let mut idx = 0;
    for &qty in quantities {
        piece_starts.push(idx);
        idx += qty;
    }
    piece_starts.push(idx);

    // Generate all options (placements)
    let mut options: Vec<Vec<Uint>> = Vec::new();

    for (shape_idx, orientations) in shapes.iter().enumerate() {
        if shape_idx >= quantities.len() || quantities[shape_idx] == 0 {
            continue;
        }

        let qty = quantities[shape_idx];
        let piece_start = piece_starts[shape_idx];

        for orientation in orientations {
            // Try all positions
            for row in 0..height {
                for col in 0..width {
                    // Check if this placement is valid (all cells within bounds)
                    let mut cells: Vec<Uint> = Vec::new();
                    let mut valid = true;

                    for &(dr, dc) in orientation {
                        let r = row as i32 + dr;
                        let c = col as i32 + dc;

                        if r < 0 || r >= height as i32 || c < 0 || c >= width as i32 {
                            valid = false;
                            break;
                        }

                        // Cell index in secondary items
                        let cell_idx = num_primary + (r as usize) * width + (c as usize);
                        cells.push(Uint(cell_idx as u64));
                    }

                    if !valid {
                        continue;
                    }

                    // Create options for each copy of this piece type
                    // Sort cells for consistent ordering
                    cells.sort();

                    for copy in 0..qty {
                        let piece_item = Uint((piece_start + copy) as u64);
                        let mut option = vec![piece_item];
                        option.extend(cells.iter().cloned());
                        options.push(option);
                    }
                }
            }
        }
    }

    if options.is_empty() {
        return total_pieces == 0;
    }

    // Create DLX problem
    // INodes::new(np, ns) where np = primary items, ns = secondary items
    let items = INodes::new(Uint(num_primary as u64), Uint(num_secondary as u64));
    // ONodes::new(n, np, ...) where n = total items, np = number of options
    let opts = ONodes::new(Uint(num_items as u64), Uint(options.len() as u64), &options, OptOrder::Seq);
    let mut problem = Problem::new(items, opts);
    let mut solver = Solver::new(&mut problem);
    let mut chooser = mrv_chooser(prefer_any(), no_tiebreak());

    // Find any solution
    solver.next_solution(&mut chooser)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (shapes, regions) = parse_input(input);

    let mut count = 0;

    for (_idx, (width, height, quantities)) in regions.iter().enumerate() {
        // Calculate total cells needed by all presents
        let total_cells: usize = quantities
            .iter()
            .enumerate()
            .map(|(i, &qty)| {
                if i < shapes.len() && !shapes[i].is_empty() {
                    qty * shapes[i][0].len()
                } else {
                    0
                }
            })
            .sum();

        let area = width * height;

        // Heuristic 1: If total cells needed > area available, region is INVALID
        if total_cells > area {
            continue;
        }

        // Count total number of presents
        let total_presents: usize = quantities.iter().sum();

        // Heuristic 2: If we have >= as many non-overlapping 3x3 spaces as presents, region is VALID
        // Since all shapes fit in a 3x3 bounding box, we can definitely place them
        let num_3x3_spaces = (width / 3) * (height / 3);
        if num_3x3_spaces >= total_presents {
            count += 1;
            continue;
        }

        // For edge cases, run DLX to verify
        if can_fit_dlx(&shapes, *width, *height, quantities) {
            count += 1;
        }
    }

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let _ = input;
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
