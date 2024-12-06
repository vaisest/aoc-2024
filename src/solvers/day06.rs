use std::collections::BTreeSet;

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
            Direction::UP => self.y -= 1,
            Direction::RIGHT => self.x += 1,
            Direction::DOWN => self.y += 1,
            Direction::LEFT => self.x -= 1,
        }
    }
    fn next_pos_by_dir(&self, dir: &Direction) -> Self {
        match dir {
            Direction::UP => Coord {
                y: self.y - 1,
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
                x: self.x - 1,
            },
        }
    }
    fn access<T: Copy>(&self, arr: &Vec<Vec<T>>) -> T {
        arr[self.y][self.x]
    }
    fn access_next_by_dir<'a, T: Copy>(
        &self,
        arr: &'a Vec<Vec<T>>,
        dir: &Direction,
    ) -> Option<&'a T> {
        let pos = self.next_pos_by_dir(dir);
        arr.get(pos.y).and_then(|it| it.get(pos.x))
    }
    fn access_next_after_turn<'a, T: Copy>(
        &self,
        arr: &'a Vec<Vec<T>>,
        dir: &Direction,
    ) -> Option<&'a T> {
        let new_dir = next_dir(dir);
        let pos = self.next_pos_by_dir(&new_dir);
        arr.get(pos.y).and_then(|it| it.get(pos.x))
    }
}

pub fn part1(input: String) -> String {
    let in_arr = input
        .lines()
        // could be arrayvec, lines are small and random access
        .map(|it| it.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // assert square
    assert!(in_arr.iter().all(|it| it.len() == in_arr.len()));

    let mut pos = Coord::new(0, 0);
    let mut dir = Direction::UP;
    'outer: for y in 0..in_arr.len() {
        for x in 0..in_arr.len() {
            if in_arr[y][x] == '^' {
                pos.set(y, x);
                break 'outer;
            }
        }
    }

    let mut visited = BTreeSet::new();
    while pos.in_bounds(in_arr.len()) {
        let elem = pos.access(&in_arr);
        if elem == '.' || elem == '^' {
            visited.insert(pos);
        }
        if pos
            .access_next_by_dir(&in_arr, &dir)
            .is_some_and(|&it| it == '#')
        {
            dir = next_dir(&dir);
        }
        pos.apply_dir(&dir);
    }

    visited.len().to_string()
}

pub fn part2(input: String) -> String {
    let in_arr = input
        .lines()
        // could be arrayvec, lines are small and access is very random
        .map(|it| it.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // assert square
    assert!(in_arr.iter().all(|it| it.len() == in_arr.len()));

    let mut pos = Coord::new(0, 0);
    let mut dir = Direction::UP;
    'outer: for y in 0..in_arr.len() {
        for x in 0..in_arr.len() {
            if in_arr[y][x] == '^' {
                pos.set(y, x);
                break 'outer;
            }
        }
    }

    let mut visited = BTreeSet::new();
    let mut cyclic_blocks = BTreeSet::new();
    while pos.in_bounds(in_arr.len()) {
        let elem = pos.access(&in_arr);
        if elem == '.' || elem == '^' {
            visited.insert(pos);
        }
        // if position in front isn't visited
        // but two position on rights are visited (or it is a turn)
        // we could start a cycle by inserting a turn
        let right_turn_dir = next_dir(&dir);
        let pos_after_turn = pos.next_pos_by_dir(&right_turn_dir);
        let pos_after_that = pos_after_turn.next_pos_by_dir(&right_turn_dir);
        // let pos_even_after_that = pos_after_that.next_pos_by_dir(&right_turn_dir);
        if visited.contains(&pos_after_turn)
            && (visited.contains(&pos_after_that) || pos_after_that.access(&in_arr) == '#')
        {
            cyclic_blocks.insert(pos.next_pos_by_dir(&dir));
        }
        if pos
            .access_next_by_dir(&in_arr, &dir)
            .is_some_and(|&it| it == '#')
        {
            dir = next_dir(&dir);
        }
        pos.apply_dir(&dir);
    }

    println!("{:?}", cyclic_blocks);

    cyclic_blocks.len().to_string()
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
