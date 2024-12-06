use std::collections::BTreeSet;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn next_dir(dir: &Direction) -> Direction {
    match dir {
        Direction::UP => Direction::RIGHT,
        Direction::RIGHT => Direction::DOWN,
        Direction::DOWN => Direction::LEFT,
        Direction::LEFT => Direction::UP,
    }
}

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
    fn get_from<'a, T: Copy>(&self, matrix: &'a Vec<Vec<T>>) -> Option<&'a T> {
        matrix.get(self.y).and_then(|it| it.get(self.x))
    }
    // fn access_next_by_dir<'a, T: Copy>(
    //     &self,
    //     arr: &'a Vec<Vec<T>>,
    //     dir: &Direction,
    // ) -> Option<&'a T> {
    //     let pos = self.next_pos_towards(dir);
    //     arr.get(pos.y).and_then(|it| it.get(pos.x))
    // }
    // fn access_next_after_turn<'a, T: Copy>(
    //     &self,
    //     arr: &'a Vec<Vec<T>>,
    //     dir: &Direction,
    // ) -> Option<&'a T> {
    //     let new_dir = next_dir(dir);
    //     let pos = self.next_pos_by_dir(&new_dir);
    //     arr.get(pos.y).and_then(|it| it.get(pos.x))
    // }
}

fn walk(mut pos: Coord, matrix: &Vec<Vec<char>>) -> BTreeSet<Coord> {
    let mut visited = BTreeSet::new();
    let mut dir = Direction::UP;
    while pos.in_bounds(matrix.len()) {
        visited.insert(pos);
        if pos
            .next_pos_towards(&dir)
            .get_from(&matrix)
            .is_some_and(|&it| it == '#')
        {
            dir = next_dir(&dir);
        }
        pos.apply_dir(&dir);
    }
    visited
}

pub fn part1(input: String) -> String {
    let matrix = input
        .lines()
        // could be arrayvec, lines are small and random access
        .map(|it| it.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // assert square
    assert!(matrix.iter().all(|it| it.len() == matrix.len()));

    let mut pos = Coord::new(0, 0);
    // find the guard's starting position
    'outer: for y in 0..matrix.len() {
        for x in 0..matrix.len() {
            if matrix[y][x] == '^' {
                pos.set(y, x);
                break 'outer;
            }
        }
    }

    walk(pos, &matrix).len().to_string()
}

fn test_for_cycle(spawn_pos: &Coord, matrix: &Vec<Vec<char>>) -> bool {
    let mut pos = spawn_pos.clone();
    let mut dir = Direction::UP;
    let mut visited = BTreeSet::new();

    while pos.in_bounds(matrix.len()) {
        visited.insert((dir, pos));

        pos.apply_dir(&dir);
        // edge case: multiple blockers near the guard -> loop
        loop {
            // move either forward or to the new direction
            // if there's a wall ahead of us, we should turn right
            if pos
                .next_pos_towards(&dir)
                .get_from(&matrix)
                .is_some_and(|&it| it == '#')
            {
                dir = next_dir(&dir);
            } else {
                break;
            }
        }
        // if we're on a visited node and going the same way,
        // we're in a loop
        if visited.contains(&(dir, pos)) {
            return true;
        }
    }
    // we went out of bounds without being in a loop
    // -> succsefully traversed so no cycle
    false
}

pub fn part2(input: String) -> String {
    let mut matrix = input
        .lines()
        // TODO: smarter container, lines are small and access is very random
        .map(|it| it.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // assert square
    assert!(matrix.iter().all(|it| it.len() == matrix.len()));

    let mut pos = Coord::new(0, 0);
    // find the guard's starting position
    'outer: for y in 0..matrix.len() {
        for x in 0..matrix.len() {
            if matrix[y][x] == '^' {
                pos.set(y, x);
                break 'outer;
            }
        }
    }
    let original_pos = pos.clone();

    // start with getting p1 answer as it's only useful
    // to place blockers on the path that the guard goes through
    let visited = walk(pos, &matrix);

    // println!()

    let mut count = 0u32;
    for coord in visited.iter() {
        if *coord == original_pos {
            continue;
        }
        if matrix[coord.y][coord.x] == '#' || matrix[coord.y][coord.x] == '^' {
            panic!("ASASDASDASASD")
        }
        matrix[coord.y][coord.x] = '#';

        if test_for_cycle(&original_pos, &matrix) {
            count += 1;
        }
        matrix[coord.y][coord.x] = '.';
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
