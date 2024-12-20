use std::{cmp::Reverse, collections::BinaryHeap};

use super::util::{adjacent_in_bounds, get_2d};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
}

fn find_shortest_path(
    area: &Vec<Vec<Tile>>,
    source: (usize, usize),
    target: (usize, usize),
) -> Option<u64> {
    // dijkstra's algorithm
    let mut costs = vec![vec![u64::MAX; area.len()]; area.len()];
    let mut heap = BinaryHeap::new();

    // 1, because the first index counts as a step
    costs[source.0][source.1] = 1;
    // Reverse helper struct to get us a min-heap instead of a max-heap
    heap.push(Reverse((1, source)));

    while let Some(Reverse((current_cost, current_pos))) = heap.pop() {
        // at each position we might want to turn to a shorter path instead of
        // going forward
        for (y, x) in adjacent_in_bounds(current_pos.0, current_pos.1, area.len()) {
            // let's not walk into a wall
            if area[y][x] == Tile::Wall {
                continue;
            }

            // avoid retrying paths where we don't get a more optimal length
            if get_2d(&costs, (y, x)).is_some_and(|&it| it <= current_cost) {
                continue;
            } else if (y, x) == target {
                return Some(current_cost);
            }
            costs[y][x] = current_cost;

            heap.push(Reverse((current_cost + 1, (y, x))));
        }
    }

    None
}

fn has_path(area: &Vec<Vec<Tile>>, source: (usize, usize), target: (usize, usize)) -> bool {
    // DFS.
    // [bool] seems to be slightly faster than a bitarr here
    let mut seen = [false; 71 * 71];
    let mut stack = vec![source];
    while let Some(current) = stack.pop() {
        let seen_idx = current.0 * area.len() + current.1;
        if seen[seen_idx] {
            continue;
        }
        seen[seen_idx] = true;
        for (y, x) in adjacent_in_bounds(current.0, current.1, area.len()) {
            // let's not walk inside a wall
            if area[y][x] == Tile::Wall {
                continue;
            }
            if (y, x) == target {
                return true;
            }
            stack.push((y, x));
        }
    }
    false
}

fn parse_input(input: String) -> (Vec<(usize, usize)>, Vec<Vec<Tile>>) {
    let mut bytes = input.lines().map(|line| {
        let (lhs, rhs) = line.split_once(',').unwrap();
        (lhs.parse::<usize>().unwrap(), rhs.parse::<usize>().unwrap())
    });
    let mut area = vec![vec![Tile::Empty; 71]; 71];

    // p1 guarantees that the first 1024 bytes don't block p2, so we can consume
    // the same amount for p1 and p2
    for (y, x) in bytes.by_ref().take(1024) {
        area[y][x] = Tile::Wall;
    }
    (bytes.collect(), area)
}
pub fn part1(input: String) -> String {
    let (_, area) = parse_input(input);
    find_shortest_path(&area, (0, 0), (70, 70))
        .expect("day 18 p1 error: no path found")
        .to_string()
}

fn index_is_tile_or_none(area: &Vec<Vec<Tile>>, y: usize, x: usize) -> bool {
    get_2d(area, (y, x)).is_none_or(|tile| match tile {
        Tile::Wall => true,
        _ => false,
    })
}
fn blocks_any_path(area: &Vec<Vec<Tile>>, y: usize, x: usize) -> bool {
    if (index_is_tile_or_none(area, y, x.wrapping_sub(1)) && index_is_tile_or_none(area, y, x + 1))
        || (index_is_tile_or_none(area, y.wrapping_sub(1), x)
            && index_is_tile_or_none(area, y + 1, x))
    {
        return true;
    }

    false
}
pub fn part2(input: String) -> String {
    let (remaining_bytes, mut area) = parse_input(input);
    for byte in remaining_bytes.into_iter() {
        area[byte.0][byte.1] = Tile::Wall;
        // we can save time by only checking for path completeness when the new
        // byte was surrounded by two blocks, which means that it blocked a way
        // through. though even with this optimisation, this is rather slow
        if blocks_any_path(&area, byte.0, byte.1) && !has_path(&area, (0, 0), (70, 70)) {
            return format!("{},{}", byte.0, byte.1);
        }
    }
    "-1".to_string()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    #[test]
    fn sample_p1() {
        use super::part1;

        // sample input uses a smaller grid, so this is different from that even
        // if it is the same input
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
        .to_string();
        assert_eq!(part1(input), "146");

        let almost_block_second_row = (0..71)
            .filter(|&x| x != 34)
            .map(|x| format!("{},{x}", 1))
            .join("\n");
        assert_eq!(part1(almost_block_second_row), "140");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let block_row_4 = (0..=70).map(|x| format!("{},{x}", 3));
        // actual input doesn't seem to have duplicates, unlike this, which has
        // the first 1024 values block the same tile
        let input = (0..1024)
            .map(|_| format!("{},{1}", 1, 1))
            .chain(block_row_4)
            .join("\n");
        assert_eq!(part2(input), "3,70");
    }
}
