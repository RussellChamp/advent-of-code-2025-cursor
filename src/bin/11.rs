advent_of_code::solution!(11);

use std::collections::HashMap;

/// Parse the input into a graph (device -> list of outputs)
fn parse_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split(':');
        let device = parts.next().unwrap().trim();
        let outputs: Vec<&str> = parts
            .next()
            .unwrap_or("")
            .split_whitespace()
            .collect();

        graph.insert(device, outputs);
    }

    graph
}

/// Count all paths from start to "out" using DFS with memoization
fn count_paths<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    // Base case: reached the destination
    if current == "out" {
        return 1;
    }

    // Check memoization
    if let Some(&cached) = memo.get(current) {
        return cached;
    }

    // Get outputs for current device
    let outputs = match graph.get(current) {
        Some(outputs) => outputs,
        None => return 0, // Dead end
    };

    // Sum paths through all outputs
    let total: u64 = outputs
        .iter()
        .map(|&next| count_paths(graph, next, memo))
        .sum();

    memo.insert(current, total);
    total
}

/// Count paths from current to "out" that visit both dac and fft
/// State: (node, visited_dac, visited_fft) where visited flags are encoded as bits
fn count_paths_through_both<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    visited_dac: bool,
    visited_fft: bool,
    memo: &mut HashMap<(&'a str, bool, bool), u64>,
) -> u64 {
    // Update visited flags based on current node
    let visited_dac = visited_dac || current == "dac";
    let visited_fft = visited_fft || current == "fft";

    // Base case: reached the destination
    if current == "out" {
        // Only count if we visited both dac and fft
        return if visited_dac && visited_fft { 1 } else { 0 };
    }

    // Check memoization
    let key = (current, visited_dac, visited_fft);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    // Get outputs for current device
    let outputs = match graph.get(current) {
        Some(outputs) => outputs,
        None => return 0, // Dead end
    };

    // Sum paths through all outputs
    let total: u64 = outputs
        .iter()
        .map(|&next| count_paths_through_both(graph, next, visited_dac, visited_fft, memo))
        .sum();

    memo.insert(key, total);
    total
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph = parse_graph(input);
    let mut memo: HashMap<&str, u64> = HashMap::new();

    let count = count_paths(&graph, "you", &mut memo);
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = parse_graph(input);
    let mut memo: HashMap<(&str, bool, bool), u64> = HashMap::new();

    let count = count_paths_through_both(&graph, "svr", false, false, &mut memo);
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2));
    }
}
