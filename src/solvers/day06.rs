use std::collections::BTreeSet;

use arrayvec::ArrayVec;
use bitvec::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
impl Direction {
    fn next_dir(&self) -> Self {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }
}

// fn next_dir(dir: &Direction) -> Direction {
//     match dir {
//         Direction::UP => Direction::RIGHT,
//         Direction::RIGHT => Direction::DOWN,
//         Direction::DOWN => Direction::LEFT,
//         Direction::LEFT => Direction::UP,
//     }
// }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Coord {
    y: usize,
    x: usize,
}
impl Coord {
    fn new(y: usize, x: usize) -> Self {
        Coord { y, x }
    }
    fn set(&mut self, y: usize, x: usize) {
        self.y = y;
        self.x = x;
    }
    fn in_bounds(&self, square_len: usize) -> bool {
        self.y < square_len && self.x < square_len
    }
    fn apply_dir(&mut self, dir: &Direction) {
        match dir {
            Direction::UP => self.y = self.y.wrapping_sub(1),
            Direction::RIGHT => self.x += 1,
            Direction::DOWN => self.y += 1,
            Direction::LEFT => self.x = self.x.wrapping_sub(1),
        }
    }
    fn next_pos_towards(&self, dir: &Direction) -> Self {
        match dir {
            Direction::UP => Coord {
                y: self.y.wrapping_sub(1),
                x: self.x,
            },
            Direction::RIGHT => Coord {
                y: self.y,
                x: self.x + 1,
            },
            Direction::DOWN => Coord {
                y: self.y + 1,
                x: self.x,
            },
            Direction::LEFT => Coord {
                y: self.y,
                x: self.x.wrapping_sub(1),
            },
        }
    }
    fn get_from<'a>(&self, matrix: &'a Matrix) -> Option<&'a Cell> {
        matrix.get(self.y).and_then(|it| it.get(self.x))
    }
}

fn input_into_matrix(input: String) -> (Coord, Matrix) {
    // returns input as a matrix using arrayvec(n=130), converted into Cell enum
    // and also reports the guard spawn point
    let mut pos = Coord::new(0, 0);

    let matrix = input
        .lines()
        .enumerate()
        .map(|(y, it)| {
            it.chars()
                .enumerate()
                .map(|(x, it)| {
                    match it {
                        '#' => Cell::Blocker,
                        '.' => Cell::Empty,
                        '^' => {
                            // we save the guard's starting position
                            pos.set(y, x);
                            Cell::Start
                        }
                        _ => unreachable!("unrecognised character in day 6 input"),
                    }
                })
                .collect()
        })
        // input is max 130x130. if using custom input, change to Vec
        .collect::<Matrix>();

    // assert that we have a square
    assert!(matrix.iter().all(|it| it.len() == matrix.len()));

    (pos, matrix)
}

fn walk(spawn_pos: &Coord, matrix: &Matrix) -> BTreeSet<Coord> {
    // finds guard's route by moving forward until we're in front of a wall and have to turn right
    let mut pos = spawn_pos.clone();
    let mut visited = BTreeSet::new();
    let mut dir = Direction::UP;
    while pos.in_bounds(matrix.len()) {
        visited.insert(pos);
        if pos
            .next_pos_towards(&dir)
            .get_from(&matrix)
            .is_some_and(|&it| it == Cell::Blocker)
        {
            dir = dir.next_dir();
        }
        pos.apply_dir(&dir);
    }
    visited
}

pub fn part1(input: String) -> String {
    let (pos, matrix) = input_into_matrix(input);

    walk(&pos, &matrix).len().to_string()
}

fn idx_for_dir(dir: &Direction) -> usize {
    match dir {
        Direction::UP => 0,
        Direction::RIGHT => 1,
        Direction::DOWN => 2,
        Direction::LEFT => 3,
    }
}

fn test_for_cycle(spawn_pos: &Coord, matrix: &Matrix) -> bool {
    let mut pos = spawn_pos.clone();
    let mut dir = Direction::UP;
    // use bitvec instead of a set. hard to read, but seems to be a 25-30x speed increase
    let mut visited = bitvec![0; matrix.len() * matrix.len() * 4];

    while pos.in_bounds(matrix.len()) {
        visited.set(
            pos.y * matrix.len() * 4 + pos.x * 4 + idx_for_dir(&dir),
            true,
        );
        pos.apply_dir(&dir);
        // edge case: multiple blockers near the guard -> loop
        loop {
            // move either forward or to the new direction
            // if there's a wall ahead of us, we should turn right
            if pos
                .next_pos_towards(&dir)
                .get_from(&matrix)
                .is_some_and(|&it| it == Cell::Blocker)
            {
                dir = dir.next_dir();
            } else {
                break;
            }
        }
        // if we're on a visited node and going the same way,
        // we're in a loop
        if pos.in_bounds(matrix.len())
            && *visited
                .get(pos.y * matrix.len() * 4 + pos.x * 4 + idx_for_dir(&dir))
                .unwrap()
        {
            return true;
        }
    }
    // we went out of bounds without being in a loop
    // -> succsefully traversed so no cycle
    false
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u8)]
enum Cell {
    Blocker,
    Empty,
    Start,
}

type Matrix = ArrayVec<ArrayVec<Cell, 130>, 130>;

pub fn part2(input: String) -> String {
    let (pos, mut matrix) = input_into_matrix(input);

    // start with getting p1 answer as it's only useful
    // to place blockers on the path that the guard goes through
    let visited = walk(&pos, &matrix);

    let mut count = 0u32;
    for coord in visited.iter() {
        // we can't block the spawn point, skip it
        if matrix[coord.y][coord.x] == Cell::Start {
            continue;
        }
        // place blocker
        matrix[coord.y][coord.x] = Cell::Blocker;

        // test-simulate if we get a loop, and count it if we do
        if test_for_cycle(&pos, &matrix) {
            count += 1;
        }
        // remove blocker
        matrix[coord.y][coord.x] = Cell::Empty;
    }
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_p1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .to_string();
        assert_eq!(part1(input), "41");
    }

    #[test]
    fn sample_p2() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .to_string();
        assert_eq!(part2(input), "6");
    }
}
