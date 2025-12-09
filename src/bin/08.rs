use rayon::slice::ParallelSliceMut;

advent_of_code::solution!(8);

/// Union-Find data structure for tracking circuits
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Already in same circuit
        }

        // Union by rank
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            }
            std::cmp::Ordering::Greater => {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
                self.rank[root_x] += 1;
            }
        }
        true
    }

    fn get_circuit_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut sizes = Vec::new();
        for i in 0..n {
            if self.find(i) == i {
                sizes.push(self.size[i]);
            }
        }
        sizes
    }
}

/// Parse a line "X,Y,Z" into coordinates
fn parse_coord(line: &str) -> Option<(i64, i64, i64)> {
    let mut parts = line.split(',');
    let x = parts.next()?.parse().ok()?;
    let y = parts.next()?.parse().ok()?;
    let z = parts.next()?.parse().ok()?;
    Some((x, y, z))
}

/// Calculate squared Euclidean distance (to avoid sqrt)
#[inline]
fn distance_squared(a: (i64, i64, i64), b: (i64, i64, i64)) -> i64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    dx * dx + dy * dy + dz * dz
}

/// Parse input and generate sorted pairs (shared between parts)
fn generate_sorted_pairs(input: &str) -> (Vec<(i64, i64, i64)>, Vec<(i64, usize, usize)>) {
    let coords: Vec<(i64, i64, i64)> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(parse_coord)
        .collect();

    let n = coords.len();

    // Generate all pairs with distances
    let mut pairs: Vec<(i64, usize, usize)> = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = distance_squared(coords[i], coords[j]);
            pairs.push((dist, i, j));
        }
    }

    // Parallel sort by distance using rayon
    pairs.par_sort_unstable_by_key(|&(dist, _, _)| dist);

    (coords, pairs)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (coords, pairs) = generate_sorted_pairs(input);
    let n = coords.len();
    if n == 0 {
        return None;
    }

    let mut uf = UnionFind::new(n);

    // Process first 1000 connections
    for &(_, i, j) in pairs.iter().take(1000) {
        uf.union(i, j);
    }

    // Get circuit sizes and find 3 largest
    let mut sizes = uf.get_circuit_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    Some(sizes.iter().take(3).map(|&s| s as u64).product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (coords, pairs) = generate_sorted_pairs(input);
    let n = coords.len();
    if n <= 1 {
        return None;
    }

    let mut uf = UnionFind::new(n);
    let mut num_circuits = n;
    let mut last_connection: Option<(usize, usize)> = None;

    for &(_, i, j) in &pairs {
        if uf.union(i, j) {
            num_circuits -= 1;
            last_connection = Some((i, j));

            if num_circuits == 1 {
                break;
            }
        }
    }

    last_connection.map(|(i, j)| coords[i].0 as u64 * coords[j].0 as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // Example uses 10 connections, not 1000 - test the core logic
        let (coords, pairs) = generate_sorted_pairs(&advent_of_code::template::read_file("examples", DAY));
        let n = coords.len();
        let mut uf = UnionFind::new(n);
        for &(_, i, j) in pairs.iter().take(10) {
            uf.union(i, j);
        }
        let mut sizes = uf.get_circuit_sizes();
        sizes.sort_unstable_by(|a, b| b.cmp(a));
        let result: u64 = sizes.iter().take(3).map(|&s| s as u64).product();
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
