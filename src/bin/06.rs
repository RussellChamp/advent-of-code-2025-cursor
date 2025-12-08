use itertools::Itertools;

advent_of_code::solution!(6);

/// Calculate result for a problem given numbers and operator
fn calculate_result(numbers: &[u64], operator: &str) -> u64 {
    match operator {
        "*" => numbers.iter().product(),
        "+" => numbers.iter().sum(),
        _ => 0,
    }
}

/// Parse problems from the worksheet and calculate the grand total
fn solve_worksheet(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    if lines.is_empty() {
        return 0;
    }

    // Parse each line into tokens using whitespace splitting
    let rows: Vec<Vec<&str>> = lines
        .iter()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();

    // Split into operator row and number rows
    let (number_rows, operator_row) = rows.split_at(rows.len() - 1);
    let operator_row = &operator_row[0];

    // Process each problem column and sum results
    (0..operator_row.len())
        .map(|problem_idx| {
            let operator = operator_row.get(problem_idx).copied().unwrap_or("+");
            let numbers: Vec<u64> = number_rows
                .iter()
                .filter_map(|row| row.get(problem_idx))
                .filter_map(|s| s.parse().ok())
                .collect();
            calculate_result(&numbers, operator)
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve_worksheet(input))
}

/// Calculate result for a problem given numbers and operator (char version)
fn calculate_result_char(numbers: &[u64], operator: char) -> u64 {
    match operator {
        '*' => numbers.iter().product(),
        '+' => numbers.iter().sum(),
        _ => 0,
    }
}

/// Find problem boundaries from column space information
/// Returns ranges of (start, end) for each problem
fn find_problem_boundaries(grid: &[Vec<char>], max_len: usize) -> Vec<(usize, usize)> {
    (0..max_len)
        .map(|col| grid.iter().all(|row| row[col] == ' '))
        .enumerate()
        .chunk_by(|&(_, is_space)| is_space)
        .into_iter()
        .filter_map(|(is_space, group)| {
            if is_space {
                None
            } else {
                let cols: Vec<_> = group.collect();
                let start = cols.first()?.0;
                let end = cols.last()?.0 + 1;
                Some((start, end))
            }
        })
        .collect()
}

/// Parse problems using cephalopod math (right-to-left columns)
/// This requires parsing the actual character grid to preserve column positions
fn solve_worksheet_cephalopod(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    if lines.is_empty() {
        return 0;
    }

    // Find max line length and build padded grid
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|l| {
            let mut chars: Vec<char> = l.chars().collect();
            chars.resize(max_len, ' ');
            chars
        })
        .collect();

    if grid.len() < 2 {
        return 0;
    }

    // Split into operator row and number rows
    let (number_rows, operator_rows) = grid.split_at(grid.len() - 1);
    let operator_row = &operator_rows[0];

    // Find problem boundaries and process each
    find_problem_boundaries(&grid, max_len)
        .into_iter()
        .map(|(start, end)| {
            // Find operator for this problem
            let operator = (start..end)
                .find_map(|col| {
                    let c = operator_row[col];
                    (c == '*' || c == '+').then_some(c)
                })
                .unwrap_or('+');

            // Read columns right-to-left, each column forms a number
            let numbers: Vec<u64> = (start..end)
                .rev()
                .filter_map(|col| {
                    let digits: String = number_rows
                        .iter()
                        .map(|row| row[col])
                        .filter(|c| c.is_ascii_digit())
                        .collect();
                    digits.parse().ok()
                })
                .collect();

            calculate_result_char(&numbers, operator)
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve_worksheet_cephalopod(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
