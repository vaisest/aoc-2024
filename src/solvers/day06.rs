use arrayvec::ArrayVec;
use rustc_hash::FxHashMap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct Coord {
    y: usize,
    x: usize,
}
impl Coord {
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
    let mut pos = Coord { y: 0, x: 0 };

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
                            pos = Coord { y, x };
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

fn walk(spawn_pos: &Coord, matrix: &Matrix) -> FxHashMap<Coord, Direction> {
    // finds guard's route by moving forward until we're in front of a wall and have to turn right
    let mut pos = spawn_pos.clone();
    let mut visited = FxHashMap::default();
    let mut dir = Direction::UP;
    while pos.in_bounds(matrix.len()) {
        loop {
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
        visited.insert(pos, dir);
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

fn test_for_cycle(
    mut pos: Coord,
    blocker_pos: Coord,
    mut direction: Direction,
    matrix: &Matrix,
) -> bool {
    // travel like in p1, but with the additional blocker, and we return true if
    // we happen to walk in the same direction in the same spot as before

    // 3D. y, x, and direction
    let mut visited = vec![false; matrix.len() * matrix.len() * 4];

    while pos.in_bounds(matrix.len()) {
        if visited[pos.y * matrix.len() * 4 + pos.x * 4 + idx_for_dir(&direction)] {
            return true;
        }
        visited[pos.y * matrix.len() * 4 + pos.x * 4 + idx_for_dir(&direction)] = true;
        // edge case: multiple blockers near the guard -> turn multiple times
        loop {
            let front = pos.next_pos_towards(&direction);
            if front
                .get_from(&matrix)
                .is_some_and(|&it| it == Cell::Blocker)
                || front == blocker_pos
            {
                direction = direction.next_dir();
            } else {
                break;
            }
        }
        pos.apply_dir(&direction);
    }
    // we went out of bounds without being in a loop
    // -> successfully traversed so no cycle
    false
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Cell {
    Blocker,
    Empty,
    Start,
}

type Matrix = ArrayVec<ArrayVec<Cell, 130>, 130>;

pub fn part2(input: String) -> String {
    let (pos, matrix) = input_into_matrix(input);

    // start with getting p1 answer as it's only useful
    // to place blockers on the path that the guard goes through
    let visited = walk(&pos, &matrix);
    // avoid getting duplicates for blockers in overlapping paths
    let mut seen = vec![false; 130 * 130];
    for (spawn_point, spawn_dir) in visited.into_iter() {
        // test-simulate if we get a loop, and count it if we do
        let blocker_spot = spawn_point.next_pos_towards(&spawn_dir);
        if blocker_spot.in_bounds(matrix.len())
            && blocker_spot
                .get_from(&matrix)
                .is_some_and(|&c| c != Cell::Blocker)
            && test_for_cycle(spawn_point, blocker_spot, spawn_dir, &matrix)
        {
            seen[blocker_spot.y * 130 + blocker_spot.x] = true;
        }
    }
    seen.into_iter().filter(|&v| v).count().to_string()
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
