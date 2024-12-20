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
    for y in 0..track.len() {
        for x in 0..track.len() {
            // we can't start from inside a wall
            if track[y][x] == Tile::Wall {
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
    minimum_cheat_advantage: i64,
) -> usize {
    let mut count = 0;
    for (target_y, row) in distances.into_iter().enumerate() {
        for (target_x, target_distance) in row.into_iter().enumerate() {
            let cheat_time = (target_y.abs_diff(source.0) + target_x.abs_diff(source.1)) as i64;
            let cheat_advantage =
                *target_distance as i64 - distances[source.0][source.1] as i64 - cheat_time;

            // p1 and p2 limit our cheat time differently
            if cheat_time >= 2 && cheat_time <= max_cheat_time
                // and we can't end up inside a wall
                && track[target_y][target_x] == Tile::Track
                && cheat_advantage >= minimum_cheat_advantage
            {
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
