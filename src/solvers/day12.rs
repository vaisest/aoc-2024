use rustc_hash::FxHashSet;

use super::util::adjacent_in_bounds;

fn traverse(
    i: usize,
    j: usize,
    matrix: &Matrix,
    mut visited: &mut FxHashSet<(usize, usize)>,
) -> (usize, usize) {
    let mut area = 1;
    let mut perimeter = 4;
    visited.insert((i, j));
    for (y, x) in adjacent_in_bounds(i, j, matrix.len()) {
        // iterate through neighbours of same character
        if matrix[y][x] != matrix[i][j] {
            continue;
        }
        // each neighbour of the same character is an open side leading to a smaller perimeter
        perimeter -= 1;

        if visited.contains(&(y, x)) {
            continue;
        }

        let (other_area, other_perimeter) = traverse(y, x, &matrix, &mut visited);
        area += other_area;
        perimeter += other_perimeter;
    }
    (area, perimeter)
}

fn parse_input(input: String) -> Matrix {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Matrix>();

    // assert square
    assert!(matrix.iter().all(|row| row.len() == matrix.len()));

    matrix
}

type Matrix = Vec<Vec<char>>;
pub fn part1(input: String) -> String {
    let matrix = parse_input(input);

    let mut visited = FxHashSet::default();
    let mut total = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            if visited.contains(&(i, j)) {
                continue;
            }
            visited.insert((i, j));
            let (area, perimeter) = traverse(i, j, &matrix, &mut visited);
            total += area * perimeter;
        }
    }

    total.to_string()
}

fn corner_conditions(
    side_one: Option<&char>,
    side_two: Option<&char>,
    corner: Option<&char>,
    this: &char,
) -> bool {
    let this = Some(this);
    // concave corner for R:
    // RR
    // AR
    if side_one == this && side_two == this && corner != this {
        return true;
    }

    // convex corner for R:
    // AR
    // AA
    if side_one != this && side_two != this && corner != this {
        return true;
    }

    // double corner for R:
    // AR
    // RA
    if side_one != this && side_two != this && corner == this {
        return true;
    }

    false
}

fn corners(i: usize, j: usize, matrix: &Matrix) -> u64 {
    let this = matrix[i][j];
    let mut count = 0;

    // not very elegant, I admit
    let top_left = matrix
        .get(i.wrapping_sub(1))
        .and_then(|row| row.get(j.wrapping_sub(1)));
    let top = matrix.get(i.wrapping_sub(1)).and_then(|row| row.get(j));
    let top_right = matrix.get(i.wrapping_sub(1)).and_then(|row| row.get(j + 1));
    let left = matrix.get(i).and_then(|row| row.get(j.wrapping_sub(1)));
    let right = matrix.get(i).and_then(|row| row.get(j + 1));
    let bottom_left = matrix.get(i + 1).and_then(|row| row.get(j.wrapping_sub(1)));
    let bottom = matrix.get(i + 1).and_then(|row| row.get(j));
    let bottom_right = matrix.get(i + 1).and_then(|row| row.get(j + 1));

    // we check how many corners there are by comparing each diagonal index with adjacent indexes near it

    // L  .
    // BL B
    if corner_conditions(left, bottom, bottom_left, &this) {
        count += 1;
    }

    // TL T
    // L  .
    if corner_conditions(left, top, top_left, &this) {
        count += 1;
    }

    // .  R
    // B BR
    if corner_conditions(right, bottom, bottom_right, &this) {
        count += 1;
    }

    // T TR
    // .  R
    if corner_conditions(right, top, top_right, &this) {
        count += 1;
    }
    count
}

fn traverse_p2(
    i: usize,
    j: usize,
    matrix: &Matrix,
    mut visited: &mut FxHashSet<(usize, usize)>,
) -> (u64, u64) {
    let mut area = 1;
    let mut corners = corners(i, j, &matrix);
    visited.insert((i, j));
    for (y, x) in adjacent_in_bounds(i, j, matrix.len()) {
        // iterate through neighbours of same character
        if matrix[y][x] != matrix[i][j] {
            continue;
        }

        if visited.contains(&(y, x)) {
            continue;
        }

        let (other_area, other_corners) = traverse_p2(y, x, &matrix, &mut visited);
        area += other_area;
        corners += other_corners;
    }
    (area, corners)
}

pub fn part2(input: String) -> String {
    let matrix = parse_input(input);

    let mut visited = FxHashSet::default();
    let mut total = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            if visited.contains(&(i, j)) {
                continue;
            }
            visited.insert((i, j));
            let (area, sides) = traverse_p2(i, j, &matrix, &mut visited);
            total += area * sides;
        }
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_p1() {
        let input = "X".to_string();
        assert_eq!(part1(input), "4");

        let input = "XX
XX"
        .to_string();
        assert_eq!(part1(input), "32");

        let input = "OX
XX"
        .to_string();
        assert_eq!(part1(input), "28");

        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
            .to_string();
        assert_eq!(part1(input), "772");

        let input = "AAAA
BBCD
BBCC
EEEC"
            .to_string();
        assert_eq!(part1(input), "140");

        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            .to_string();
        assert_eq!(part1(input), "1930");
    }

    #[test]
    fn sample_p2() {
        let input = "X".to_string();
        assert_eq!(part1(input), "4");

        let input = "AAAA
BBCD
BBCC
EEEC"
            .to_string();
        assert_eq!(part2(input), "80");

        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"
            .to_string();
        assert_eq!(part2(input), "236");

        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"
            .to_string();
        assert_eq!(part2(input), "368");

        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            .to_string();
        assert_eq!(part2(input), "1206");
    }
}
