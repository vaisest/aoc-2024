use super::util::Direction;

#[derive(Clone, Copy)]
enum AreaElement {
    Wall,
    Box,
    Box2,
    Empty,
}

fn try_move_box(
    matrix: &mut Vec<Vec<AreaElement>>,
    coord: (usize, usize),
    direction: &Direction,
    moving_box: bool,
    // return true -> bot moves
    // return false -> bot doesn't move (hits wall)
) -> bool {
    let front = direction.apply_unchecked(coord);
    match matrix[front.0][front.1] {
        AreaElement::Wall => {
            return false;
        }
        AreaElement::Empty => {
            if moving_box {
                matrix[front.0][front.1] = AreaElement::Box
            }
            return true;
        }
        AreaElement::Box => {
            if try_move_box(matrix, front, direction, true) {
                if !moving_box {
                    matrix[front.0][front.1] = AreaElement::Empty;
                }
                true
            } else {
                // bot encountered a wall so nothing moves
                false
            }
        }
        _ => unreachable!(),
    }
}
fn gps_coord(coord: (usize, usize)) -> usize {
    100 * coord.0 + coord.1
}
pub fn part1(input: String) -> String {
    let (area_text, commands_text) = input.split_once("\n\n").unwrap();
    let mut robot_coord = (0, 0);
    let mut area = area_text
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => AreaElement::Wall,
                    'O' => AreaElement::Box,
                    '.' => AreaElement::Empty,
                    '@' => {
                        robot_coord = (y, x);
                        AreaElement::Empty
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let commands = commands_text.chars().filter_map(|c| match c {
        '^' => Some(Direction::Up),
        'v' => Some(Direction::Down),
        '<' => Some(Direction::Left),
        '>' => Some(Direction::Right),
        '\n' => None,
        _ => unreachable!("malformed input"),
    });

    for command in commands.into_iter() {
        if try_move_box(&mut area, robot_coord, &command, false) {
            robot_coord = command.apply_unchecked(robot_coord);
        }
    }

    // for line in area.iter() {
    //     println!(
    //         "{:?}",
    //         line.iter()
    //             .map(|it| match it {
    //                 AreaElement::Wall => '#',
    //                 AreaElement::Empty => '.',
    //                 AreaElement::Box => 'O',
    //             })
    //             .join("")
    //     )
    // }

    area.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, elem)| match elem {
                    AreaElement::Box => gps_coord((y, x)),
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn move_p2(
    area: &mut Vec<Vec<AreaElement>>,
    direction: &Direction,
    coord: (usize, usize),
    // up/down moves require two passes as otherwise
    // we might only move half the boxes
    check_only: bool,
) -> bool {
    match area[coord.0][coord.1] {
        AreaElement::Empty => true,
        AreaElement::Wall => false,
        AreaElement::Box => {
            let front = direction.apply_unchecked(coord);
            let res = move_p2(area, direction, front, check_only)
                && move_p2(area, direction, (front.0, front.1 + 1), check_only);
            if res && !check_only {
                area[coord.0][coord.1] = AreaElement::Empty;
                area[coord.0][coord.1 + 1] = AreaElement::Empty;
                area[front.0][front.1] = AreaElement::Box;
                area[front.0][front.1 + 1] = AreaElement::Box2;
                return true;
            }
            res
        }
        AreaElement::Box2 => {
            let front = direction.apply_unchecked(coord);
            let res = move_p2(area, direction, front, check_only)
                && move_p2(area, direction, (front.0, front.1 - 1), check_only);
            if res && !check_only {
                area[coord.0][coord.1 - 1] = AreaElement::Empty;
                area[coord.0][coord.1] = AreaElement::Empty;
                area[front.0][front.1 - 1] = AreaElement::Box;
                area[front.0][front.1] = AreaElement::Box2;
                return true;
            }
            res
        }
    }
}

fn move_lr(area: &mut Vec<Vec<AreaElement>>, direction: &Direction, coord: (usize, usize)) -> bool {
    match area[coord.0][coord.1] {
        AreaElement::Empty => true,
        AreaElement::Wall => false,
        AreaElement::Box => {
            let next = direction.apply_unchecked(coord);
            if move_lr(area, direction, next) {
                area[coord.0][coord.1] = AreaElement::Empty;
                area[next.0][next.1] = AreaElement::Box;
                return true;
            }
            false
        }
        AreaElement::Box2 => {
            let next = direction.apply_unchecked(coord);
            if move_lr(area, direction, next) {
                area[coord.0][coord.1] = AreaElement::Empty;
                area[next.0][next.1] = AreaElement::Box2;
                return true;
            }
            false
        }
    }
}

pub fn part2(input: String) -> String {
    let (area_text, commands_text) = input.split_once("\n\n").unwrap();
    let mut robot_coord = (0, 0);
    let mut area = area_text
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let mut row = vec![];
            line.chars().enumerate().for_each(|(x, c)| {
                let elem = match c {
                    '#' => [AreaElement::Wall; 2],
                    'O' => [AreaElement::Box, AreaElement::Box2],
                    '.' => [AreaElement::Empty; 2],
                    '@' => {
                        robot_coord = (y, 2 * x);
                        [AreaElement::Empty; 2]
                    }
                    _ => unreachable!(),
                };
                row.extend_from_slice(&elem);
            });
            row
        })
        .collect::<Vec<Vec<_>>>();

    let commands = commands_text.chars().filter_map(|c| match c {
        '^' => Some(Direction::Up),
        'v' => Some(Direction::Down),
        '<' => Some(Direction::Left),
        '>' => Some(Direction::Right),
        '\n' => None,
        _ => unreachable!("malformed input"),
    });

    for direction in commands {
        let next_spot = direction.apply_unchecked(robot_coord);
        let moved = match direction {
            Direction::Down | Direction::Up => {
                if move_p2(&mut area, &direction, next_spot, true) {
                    move_p2(&mut area, &direction, next_spot, false)
                } else {
                    false
                }
            }
            Direction::Left | Direction::Right => move_lr(&mut area, &direction, next_spot),
        };
        if moved {
            robot_coord = next_spot;
        }
    }

    area.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, elem)| match elem {
                    AreaElement::Box => gps_coord((y, x)),
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_p1() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
            .to_string();
        assert_eq!(part1(input), "2028");

        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            .to_string();
        assert_eq!(part1(input), "10092");
    }

    #[test]
    fn sample_p2() {
        let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"
            .to_string();
        assert_eq!(part2(input), "618");

        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            .to_string();
        assert_eq!(part2(input), "9021");

        let input = "#######
#.....#
#.OO@.#
#.....#
#######

<<"
        .to_string();
        assert_eq!(part2(input), "406");

        let input = "#######
#.....#
#.O#..#
#..O@.#
#.....#
#######

<v<<^"
            .to_string();
        assert_eq!(part2(input), "509");
    }
}
