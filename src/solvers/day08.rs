use std::{
    collections::{BTreeMap, HashSet},
    i32,
};

use itertools::Itertools;

type Matrix = Vec<Vec<char>>;

fn distance_pair(lhs: &(usize, usize), rhs: &(usize, usize)) -> (i32, i32) {
    return (rhs.0 as i32 - lhs.0 as i32, rhs.1 as i32 - lhs.1 as i32);
}
fn in_bounds_of(matrix_len: usize, coord: (i32, i32)) -> bool {
    coord.0 >= 0 && coord.0 < matrix_len as i32 && coord.1 >= 0 && coord.1 < matrix_len as i32
}
fn parse_input(input: String) -> (Matrix, BTreeMap<char, Vec<(usize, usize)>>) {
    let matrix = input
        .lines()
        .map(|it| it.chars().collect())
        .collect::<Matrix>();

    assert!(matrix.iter().all(|it| it.len() == matrix.len()));

    let mut unique_chars: BTreeMap<char, Vec<(usize, usize)>> = BTreeMap::new();
    for y in 0..matrix.len() {
        for x in 0..matrix.len() {
            let c = matrix[y][x];
            if c != '.' {
                unique_chars.entry(c).or_default().push((y, x));
            }
        }
    }
    (matrix, unique_chars)
}
fn p1_line_positions(
    first: &(usize, usize),
    second: &(usize, usize),
    matrix_len: usize,
) -> Vec<(usize, usize)> {
    // a pair of positions defines a line, and
    // we mark the first position that's as far away from each position as the
    // two positions are from each other
    let mut out = Vec::new();

    let (dy, dx) = distance_pair(first, second);

    let first_coord = ((first.0 as i32 - dy), (first.1 as i32 - dx));
    if in_bounds_of(matrix_len, first_coord) {
        out.push((first_coord.0 as usize, first_coord.1 as usize));
    }
    let second_coord = ((second.0 as i32 + dy), (second.1 as i32 + dx));
    if in_bounds_of(matrix_len, second_coord) {
        out.push((second_coord.0 as usize, second_coord.1 as usize));
    }
    out
}
pub fn part1(input: String) -> String {
    let (matrix, unique_chars) = parse_input(input);
    let mut anti_nodes = HashSet::new();

    for (_, positions) in unique_chars {
        for pair in positions.iter().combinations(2) {
            anti_nodes.extend(p1_line_positions(&pair[0], &pair[1], matrix.len()));
        }
    }

    anti_nodes.len().to_string()
}

fn p2_line_positions(
    first: &(usize, usize),
    second: &(usize, usize),
    matrix_len: usize,
) -> Vec<(usize, usize)> {
    // like p1_line_positions, except we mark the nodes themselves (mul == 0)
    // and repeating valid distances from the nodes (mul > 1)
    let mut out = Vec::new();

    let (dy, dx) = distance_pair(first, second);

    let mut mul = 0;
    loop {
        let dy_scaled = dy * mul;
        let dx_scaled = dx * mul;
        let len_before = out.len();

        let first_coord = ((first.0 as i32 + dy_scaled), (first.1 as i32 + dx_scaled));
        if in_bounds_of(matrix_len, first_coord) {
            out.push((first_coord.0 as usize, first_coord.1 as usize));
        }
        let second_coord = ((second.0 as i32 - dy_scaled), (second.1 as i32 - dx_scaled));
        if in_bounds_of(matrix_len, second_coord) {
            out.push((second_coord.0 as usize, second_coord.1 as usize));
        }
        if len_before == out.len() {
            break;
        }
        mul += 1;
    }
    out
}

pub fn part2(input: String) -> String {
    let (matrix, unique_chars) = parse_input(input);
    let mut anti_nodes = HashSet::new();

    for (_, positions) in unique_chars {
        for pair in positions.iter().combinations(2) {
            anti_nodes.extend(p2_line_positions(&pair[0], &pair[1], matrix.len()));
        }
    }

    anti_nodes.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_p1() {
        let input = "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
.........."
            .to_string();
        assert_eq!(part1(input), "2");

        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            .to_string();
        assert_eq!(part1(input), "14");
    }

    #[test]
    fn sample_p2() {
        let input = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
.........."
            .to_string();
        assert_eq!(part2(input), "9");

        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            .to_string();
        assert_eq!(part2(input), "34");
    }
}
