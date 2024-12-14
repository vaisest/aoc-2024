use bitvec::prelude::*;

fn adjacent_in_bounds(
    i: usize,
    j: usize,
    matrix_len: usize,
) -> impl Iterator<Item = (usize, usize)> {
    const ADJACENTS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)]; // up down left right
    return ADJACENTS.iter().filter_map(move |(dy, dx)| {
        let pair = (i as i32 + *dy, j as i32 + *dx);
        let legal_range = 0..(matrix_len as i32);
        if legal_range.contains(&pair.0) && legal_range.contains(&pair.1) {
            return Some((pair.0 as usize, pair.1 as usize));
        } else {
            return None;
        }
    });
}

fn traverse(i: usize, j: usize, matrix: &Matrix) -> u64 {
    // simple counting bfs
    let mut seen = bitvec![0; matrix.len() * matrix.len()];
    let mut queue = Vec::new();

    seen.set(i * matrix.len() + j, true);
    queue.push((i, j));

    let mut total = 0;
    while !queue.is_empty() {
        let (y, x) = queue.pop().unwrap();
        let current_elev = matrix[y][x];

        // our goal is elevation 9
        if matrix[y][x] == 9 {
            total += 1;
            continue;
        }

        for adj in adjacent_in_bounds(y, x, matrix.len()) {
            if !seen.get(adj.0 * matrix.len() + adj.1).unwrap()
                // we only visit nodes if they're a gradual slope, i.e. 1 higher in elevation
                && matrix[adj.0][adj.1] == current_elev + 1
            {
                seen.set(adj.0 * matrix.len() + adj.1, true);
                queue.push(adj);
            }
        }
    }
    total
}

fn traverse_simple(i: usize, j: usize, matrix: &Matrix) -> u64 {
    // recursively travers in all adjacent valid directions
    // with base case where node equals 9
    let current_elevation = matrix[i][j];
    if current_elevation == 9 {
        return 1;
    }

    let mut total = 0;

    for (y, x) in adjacent_in_bounds(i, j, matrix.len()) {
        if matrix[y][x] == current_elevation + 1 {
            total += traverse_simple(y, x, matrix);
        }
    }

    total
}

type Matrix = Vec<Vec<u32>>;
pub fn part1(input: String) -> String {
    let matrix = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Matrix>();

    // assert square input
    assert!(matrix.iter().all(|row| row.len() == matrix.len()));

    let mut count = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            if matrix[i][j] == 0 {
                count += traverse(i, j, &matrix);
            }
        }
    }

    count.to_string()
}

pub fn part2(input: String) -> String {
    let matrix = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Matrix>();

    // assert square input
    assert!(matrix.iter().all(|row| row.len() == matrix.len()));

    let mut count = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            if matrix[i][j] == 0 {
                count += traverse_simple(i, j, &matrix);
            }
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_p1() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            .to_string();
        assert_eq!(part1(input), "36");
    }

    #[test]
    fn sample_p2() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            .to_string();
        assert_eq!(part2(input), "81");
    }
}
