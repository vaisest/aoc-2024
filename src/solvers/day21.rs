use itertools::Itertools;
use rustc_hash::FxHashMap;

fn numpad_pos(c: char) -> (i32, i32) {
    // 7 8 9
    // 4 5 6
    // 1 2 3
    //   0 A
    match c {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '0' => (3, 1),
        'A' => (3, 2),
        _ => unreachable!(),
    }
}
fn dirpad_pos(c: char) -> (i32, i32) {
    //   ^ A
    // < v >
    match c {
        '^' => (0, 1),
        'A' => (0, 2),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _ => {
            println!("{c}");
            unreachable!()
        }
    }
}
fn move_vertical(y: i32, y_target: i32, output: &mut String) {
    if y == y_target {
        return;
    }
    let count = y.abs_diff(y_target);
    let c = if y < y_target { 'v' } else { '^' };
    (0..count).for_each(|_| output.push(c));
}
fn move_horizontal(x: i32, x_target: i32, output: &mut String) {
    if x == x_target {
        return;
    }
    let count = x.abs_diff(x_target);
    let c = if x < x_target { '>' } else { '<' };
    (0..count).for_each(|_| output.push(c));
}
fn dirpad_path_to(c: char, state: &mut (i32, i32), output: &mut String) {
    // the order of movement we want to execute is left/up/down/right in order
    // of preference [1] while also avoiding going over the forbidden spot,
    // which doesn't have a button and would break our bots

    // [1]: https://old.reddit.com/r/adventofcode/comments/1hjgyps/2024_day_21_part_2_i_got_greedyish/

    let target = dirpad_pos(c);
    // starting from (1, 0) => avoid going over forbidden spot
    if state.1 == 0 {
        move_horizontal(state.1, target.1, output);
        move_vertical(state.0, target.0, output);
    // going to (1, 0) => avoid going over forbidden spot
    } else if target.1 == 0 {
        move_vertical(state.0, target.0, output);
        move_horizontal(state.1, target.1, output);
        // going left
    } else if target.1 < state.1 {
        move_horizontal(state.1, target.1, output);
        move_vertical(state.0, target.0, output);
    } else {
        move_vertical(state.0, target.0, output);
        move_horizontal(state.1, target.1, output);
    }

    *state = target;
}
fn numpad_path_to(c: char, state: &mut (i32, i32), output: &mut String) {
    // the order of movement we want to execute is left/up/down/right in order
    // of preference [1] while also avoiding going over the forbidden spot,
    // which doesn't have a button and would break our bots

    // [1]: https://old.reddit.com/r/adventofcode/comments/1hjgyps/2024_day_21_part_2_i_got_greedyish/
    let target = numpad_pos(c);

    //starting from (_, 0) (left col) and going to (3, _) (bottom row) => avoid
    // going over forbidden spot
    if state.1 == 0 && target.0 == 3 {
        move_horizontal(state.1, target.1, output);
        move_vertical(state.0, target.0, output);
    // starting from (3, _) (bottom row) and going to (_, 0) (left col) => avoid
    // going over forbidden spot
    } else if state.0 == 3 && target.1 == 0 {
        move_vertical(state.0, target.0, output);
        move_horizontal(state.1, target.1, output);
    } else if target.1 < state.1 {
        move_horizontal(state.1, target.1, output);
        move_vertical(state.0, target.0, output);
    } else {
        move_vertical(state.0, target.0, output);
        move_horizontal(state.1, target.1, output);
    }
    *state = target;
}
fn numpad_moves(code: &str) -> String {
    let mut output = String::new();
    // robot starts on the button 'A'
    let mut state = numpad_pos('A');
    for number in code.chars() {
        // for each part of the code (including the 'A'), we navigate the
        // robotic finger to that button, and press it (A)
        numpad_path_to(number, &mut state, &mut output);
        output.push('A');
    }
    output
}

fn calculate_button_press_count(numpad_result: String, iterations_after_first: u32) -> usize {
    // robot starts on the button 'A'
    let chars = ['<', '>', '^', 'v', 'A'];
    // pre-calculate amount of button presses needed for all moves (i.e. from
    // button '>' to '<')
    let mut count_map = FxHashMap::from_iter(chars.iter().permutations(2).map(|vec| {
        let mut output = String::new();
        dirpad_path_to(*vec[1], &mut dirpad_pos(*vec[0]), &mut output);
        output.push('A');
        ((*vec[0], *vec[1]), output.len())
    }));

    // sum the cost of a set of movements on a direction pad, starting from 'A'
    let sum_path = |moves: String, count_map: &FxHashMap<(char, char), usize>| {
        // intermediate bots always start and end on the 'A' button
        let mut prev = 'A';
        let mut n = 0;
        for c in moves.chars() {
            n += if prev == c {
                // moving to the same button would simply indicate another 'A'
                // press when the bots are lined up
                1
            } else {
                *count_map.get(&(prev, c)).unwrap()
            };
            prev = c;
        }
        n
    };

    let mut count_map2 = FxHashMap::default();
    for _ in 0..iterations_after_first {
        chars.iter().permutations(2).for_each(|vec| {
            let mut output = String::new();
            // get base route to other to get characters to iterate over
            dirpad_path_to(*vec[1], &mut dirpad_pos(*vec[0]), &mut output);
            output.push('A');

            let n = sum_path(output, &count_map);
            count_map2.insert((*vec[0], *vec[1]), n);
        });

        std::mem::swap(&mut count_map, &mut count_map2);
        count_map2.clear();
    }

    // finally, we apply the calculated move costs to the original string
    sum_path(numpad_result, &count_map)
}

fn calculate_complexity(code: &str, robot_count: u32) -> usize {
    // first we get an actual set of moves (like "<A^A>^^AvvvA")
    let numpad_result = numpad_moves(code);
    // and then we calculate how many moves we would need to perform based on
    // the count of the intermediate robots
    let intermediate_costs = calculate_button_press_count(numpad_result, robot_count - 1);
    code.strip_suffix('A').unwrap().parse::<usize>().unwrap() * intermediate_costs
}

pub fn part1(input: String) -> String {
    let codes = input.lines().map(|line| line);

    codes
        .map(|code| calculate_complexity(code, 2))
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let codes = input.lines().map(|line| line);

    codes
        .map(|code| calculate_complexity(code, 25))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = "029A
980A
179A
456A
379A"
            .to_string();
        assert_eq!(part1(input), "126384");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = "029A
980A
179A
456A
379A"
            .to_string();
        assert_eq!(part2(input), "154115708116294");
    }
}
