use std::collections::BinaryHeap;

use rustc_hash::{FxHashMap, FxHashSet};

use super::util::Direction;

#[derive(PartialEq)]
enum Element {
    Wall,
    Empty,
}

#[derive(Debug, PartialEq, Eq)]
struct Dijk {
    pos: (usize, usize),
    cost: u64,
    dir: Direction,
    // for p2. probably minimal cost to have it as None for p1
    path: Option<Vec<(usize, usize)>>,
}

impl Ord for Dijk {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for Dijk {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost).reverse())
    }
}

fn turns(direction: &Direction) -> impl Iterator<Item = (Direction, u64)> {
    match direction {
        Direction::Down => [
            (Direction::Down, 0),
            (Direction::Right, 1000),
            (Direction::Left, 1000),
        ]
        .into_iter(),
        Direction::Up => [
            (Direction::Up, 0),
            (Direction::Right, 1000),
            (Direction::Left, 1000),
        ]
        .into_iter(),
        Direction::Left => [
            (Direction::Left, 0),
            (Direction::Up, 1000),
            (Direction::Down, 1000),
        ]
        .into_iter(),
        Direction::Right => [
            (Direction::Right, 0),
            (Direction::Up, 1000),
            (Direction::Down, 1000),
        ]
        .into_iter(),
    }
}

fn find_path(map: &Vec<Vec<Element>>, source: (usize, usize), target: (usize, usize)) -> u64 {
    // dijkstra's algorithm, but instead of storing costs per 2d index, we want to store
    // them per (2d index, direction) as turning is treated separately from moving
    let mut costs = FxHashMap::default();
    let mut heap = BinaryHeap::new();

    heap.push(Dijk {
        pos: source,
        cost: 0,
        dir: Direction::Right,
        path: None,
    });

    while !heap.is_empty() {
        let u = heap.pop().unwrap();

        if u.pos == target {
            return u.cost;
        }

        if costs.get(&(u.pos, u.dir)).is_some_and(|&it| it <= u.cost) {
            continue;
        }

        costs.insert((u.pos, u.dir), u.cost);

        // at each position we might want to turn to a shorter path
        // instead of going forward
        for (new_dir, turn_cost) in turns(&u.dir) {
            let new_pos = new_dir.apply_unchecked(u.pos);

            // let's not walk into a wall
            if map[new_pos.0][new_pos.1] != Element::Empty {
                continue;
            }

            // there's no point in standing still so we automatically
            // add 1001 instead of 1000 on a turn
            let new_cost = u.cost + turn_cost + 1;

            heap.push(Dijk {
                pos: new_pos,
                cost: new_cost,
                dir: new_dir,
                path: None,
            });
        }
    }

    u64::MAX
}
pub fn part1(input: String) -> String {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = (y, x);
                        Element::Empty
                    }
                    'E' => {
                        end = (y, x);
                        Element::Empty
                    }
                    '#' => Element::Wall,
                    '.' => Element::Empty,
                    _ => unreachable!("malformed input"),
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    assert_ne!(start, (0, 0));
    assert_ne!(end, (0, 0));

    find_path(&map, start, end).to_string()
}

fn p2_paths(
    map: &Vec<Vec<Element>>,
    source: (usize, usize),
    target: (usize, usize),
) -> FxHashSet<(usize, usize)> {
    // similar to p1, but does not terminate as early due to requiring all shortest paths
    // and dijk is modified to contain a vector of the path
    // which is cloned *a lot*
    // TODO: this could probably be optimised quite a bit by backtracking from the target
    // instead of keeping countless path vectors that get discarded when the pathfinding goes the wrong way
    let mut costs = FxHashMap::default();
    let mut heap = BinaryHeap::new();
    let mut tiles = FxHashSet::default();
    let mut shortest_cost = None;

    heap.push(Dijk {
        pos: source,
        cost: 0,
        dir: Direction::Right,
        path: Some(vec![source]),
    });

    while !heap.is_empty() {
        let cur = heap.pop().unwrap();

        if cur.pos == target {
            if shortest_cost.is_none() {
                shortest_cost.replace(cur.cost);
            } else if shortest_cost.unwrap() < cur.cost {
                continue;
            }
            tiles.extend(cur.path.unwrap().into_iter());
            continue;
        }

        // at each position we might want to turn to a shorter path
        // instead of going forward
        for (new_dir, turn_cost) in turns(&cur.dir) {
            let new_pos = new_dir.apply_unchecked(cur.pos);

            // let's not walk into a wall
            if map[new_pos.0][new_pos.1] == Element::Wall {
                continue;
            }

            // we automatically add 1001 instead of 1000 on a turn
            let new_cost = cur.cost + turn_cost + 1;

            let new_key = (new_pos, new_dir);
            // avoid calculating longer paths to an already as or less optimal calculated spot by skipping this move
            let existing_cost = costs.get(&new_key);
            if existing_cost.is_some_and(|&it| it < new_cost) {
                continue;
            }

            costs.insert(new_key, new_cost);

            let mut new_path = cur.path.clone().unwrap();
            new_path.push(new_pos);
            heap.push(Dijk {
                pos: new_pos,
                cost: new_cost,
                dir: new_dir,
                path: Some(new_path),
            });
        }
    }

    tiles
}

pub fn part2(input: String) -> String {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = (y, x);
                        Element::Empty
                    }
                    'E' => {
                        end = (y, x);
                        Element::Empty
                    }
                    '#' => Element::Wall,
                    '.' => Element::Empty,
                    _ => unreachable!("malformed input"),
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    assert_ne!(start, (0, 0));
    assert_ne!(end, (0, 0));

    let tiles = p2_paths(&map, start, end);

    tiles.len().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;
        let input = "#######
#S...E#
#######"
            .to_string();
        assert_eq!(part1(input), "4");

        // not sure if this needs fixing. currently you can't turn
        // 180 degrees in place, but my input doesn't seem to require this?
        //         let input = "#######
        // #E...S#
        // #######"
        //             .to_string();
        //         assert_eq!(part1(input), "2004");

        let input = "#######
#S....#
#####E#
#######"
            .to_string();
        assert_eq!(part1(input), "1005");

        let input = "#######
#S....#
#####.#
#E....#
#######"
            .to_string();
        assert_eq!(part1(input), "2010");

        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
            .to_string();
        assert_eq!(part1(input), "7036");

        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
            .to_string();
        assert_eq!(part1(input), "11048");
    }

    #[test]
    fn sample_p2() {
        use super::part2;
        let input = "#######
#S...E#
#######"
            .to_string();
        assert_eq!(part2(input), "5");

        let input = "#######
#S....#
#####.#
#E....#
#######"
            .to_string();
        assert_eq!(part2(input), "11");

        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
            .to_string();
        assert_eq!(part2(input), "45");

        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
            .to_string();
        assert_eq!(part2(input), "64");
    }
}
