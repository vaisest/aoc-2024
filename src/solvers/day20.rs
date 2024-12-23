use crate::solvers::util::adjacent_in_bounds;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Track,
    Wall,
}
struct RaceState {
    pos: (usize, usize),
    time: u64,
}

fn sum_all_cheats(
    track: &Vec<Vec<Tile>>,
    distances: &Vec<Vec<u64>>,
    max_cheat_time: i64,
    minimum_cheat_advantage: i64,
) -> usize {
    // TODO: optimise this as it is rather wasteful
    let mut result = 0;
    for y in 1..track.len() - 1 {
        for x in 1..track.len() - 1 {
            // we can't start from inside a wall
            if distances[y][x] == u64::MAX {
                continue;
            }
            result += cheat_advantages(
                &track,
                (y, x),
                &distances,
                max_cheat_time,
                minimum_cheat_advantage,
            );
        }
    }
    result
}

fn cheat_advantages(
    track: &Vec<Vec<Tile>>,
    source: (usize, usize),
    distances: &Vec<Vec<u64>>,
    max_cheat_time: i64,
    required_cheat_advantage: i64,
) -> usize {
    // a lot of the cheats produced overlap for different source points, which
    // means it might be possible to cache them
    let mut count = 0;
    let legal_range = 0..track.len() as i64;

    // we want to have a total of up to max_cheat_time spread between dy and dx
    for dy in (-max_cheat_time)..=max_cheat_time {
        let target_y = source.0 as i64 + dy;
        let remainder = max_cheat_time - dy.abs();
        for dx in (-remainder)..=remainder {
            let target_x = source.1 as i64 + dx;

            if !legal_range.contains(&target_y) || !legal_range.contains(&target_x) {
                continue;
            }
            let target_y = target_y as usize;
            let target_x = target_x as usize;

            let target_distance = distances[target_y][target_x];
            // if target is a wall, there's no point calculating anything else
            if target_distance == u64::MAX {
                continue;
            }

            let source_distance = distances[source.0][source.1];
            let cheat_time = dy.abs() + dx.abs();
            let cheat_advantage = target_distance as i64 - source_distance as i64 - cheat_time;

            if cheat_advantage >= required_cheat_advantage {
                count += 1;
            }
        }
    }
    count
}

fn parse_input(input: String) -> (Vec<Vec<Tile>>, (usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let track = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Track,
                    'S' => {
                        start = (y, x);
                        Tile::Track
                    }
                    'E' => {
                        end = (y, x);
                        Tile::Track
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect::<Vec<Vec<Tile>>>();

    assert_ne!(start, (0, 0));
    assert_ne!(end, (0, 0));
    // assert square
    assert!(track.iter().all(|row| row.len() == track.len()));
    (track, start, end)
}

fn calculate_distances(
    track: &Vec<Vec<Tile>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<Vec<u64>> {
    let mut state = RaceState {
        pos: start,
        time: 0,
    };

    let mut distances = vec![vec![u64::MAX; track.len()]; track.len()];
    distances[start.0][start.1] = 0;

    while state.pos != end {
        for pos in adjacent_in_bounds(state.pos.0, state.pos.1, track.len()) {
            // let's not crash
            if track[pos.0][pos.1] == Tile::Wall {
                continue;
            }

            if distances[pos.0][pos.1] == u64::MAX {
                distances[pos.0][pos.1] = state.time + 1;

                state = RaceState {
                    pos,
                    time: state.time + 1,
                };
            }
        }
    }
    distances
}

pub fn part1(input: String) -> String {
    let (track, start, end) = parse_input(input);
    let distances = calculate_distances(&track, start, end);
    sum_all_cheats(&track, &distances, 2, 100).to_string()
}

pub fn part2(input: String) -> String {
    let (track, start, end) = parse_input(input);
    let distances = calculate_distances(&track, start, end);
    sum_all_cheats(&track, &distances, 20, 100).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"
            .to_string();
        // input too small for result to be higher
        assert_eq!(part1(input), "0");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"
            .to_string();
        assert_eq!(part2(input), "0");
    }
}
