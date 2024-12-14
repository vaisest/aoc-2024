use regex::Regex;

fn gaussian_elimination(mut matrix: [[f64; 3]; 2]) -> (f64, f64) {
    // our matrix is
    // ax bx | x
    // ay by | y

    // multiplier: ay / ax
    let f = matrix[1][0] / matrix[0][0];
    // we negate f * R_1 from R_2, eliminating by

    // ay -= (ay / ax) * ax <=> ay = 0
    // by -= (ay / ax) * bx
    // y  -= (ay / ax) * x
    matrix[1][0] = 0.0;
    matrix[1][1] -= f * matrix[0][1];
    matrix[1][2] -= f * matrix[0][2];

    // multiplier: bx / by
    let f = matrix[0][1] / matrix[1][1];
    // we negate f * R_2 from R_1, eliminating bx

    // x  -= (bx / by) * y
    // bx -= (bx / by) * by <=> bx = 0
    // ax -= (bx / by) * ay
    matrix[0][2] -= f * matrix[1][2];
    matrix[0][1] = 0.0;
    matrix[0][0] -= f * matrix[1][0];

    // now we have to scale both rows so that positions ax and by are equal to 1,
    // which gets us our answer in x and y
    (matrix[0][2] / matrix[0][0], matrix[1][2] / matrix[1][1])
}

fn close_enough(n: f64, threshold_exp: i32) -> Option<u64> {
    // essentially python math.isclose() which checks if this is basically an integer
    // there is a very tiny bit of inaccuracy from gaussian_elimination as it has to use floats.
    let rounded = n.round();
    if (rounded - n).abs() < 10.0f64.powi(threshold_exp) {
        Some(rounded as u64)
    } else {
        None
    }
}

fn solve(input: String, constant: f64, threshold_exp: i32) -> u64 {
    let button_re = Regex::new(r"Button .: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    input
        .split("\n\n")
        // trailing newline
        .filter(|&it| it != "")
        .map(|block| {
            let mut it = block.lines();
            let button_a = it.next().unwrap();
            let button_b = it.next().unwrap();
            let prize = it.next().unwrap();
            let (_, [ax, ay]) = button_re.captures(button_a).unwrap().extract();
            let (_, [bx, by]) = button_re.captures(button_b).unwrap().extract();
            let (_, [x, y]) = prize_re.captures(prize).unwrap().extract();

            let matrix = [
                // augmented matrix
                // ax bx | x
                // ay by | y
                [
                    ax.parse().unwrap(),
                    bx.parse().unwrap(),
                    constant + x.parse::<f64>().unwrap(),
                ],
                [
                    ay.parse().unwrap(),
                    by.parse().unwrap(),
                    constant + y.parse::<f64>().unwrap(),
                ],
            ];
            gaussian_elimination(matrix)
        })
        .filter_map(|pair| {
            // I wish I had an if-let chain :(

            // threshold has to be adjustable as p2 numbers are too inaccurate,
            close_enough(pair.0, threshold_exp).and_then(|lhs| {
                close_enough(pair.1, threshold_exp).and_then(|rhs| Some(lhs * 3 + rhs))
            })
        })
        .sum::<u64>()
}

pub fn part1(input: String) -> String {
    solve(input, 0.0, -9).to_string()
}

pub fn part2(input: String) -> String {
    // threshold has to be higher due to big numbers... kind of stupid
    solve(input, 10000000000000.0, -3).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_p1() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
            .to_string();
        assert_eq!(part1(input), "480");
    }

    #[test]
    fn sample_p2() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
            .to_string();
        assert_eq!(part2(input), "875318608908");
    }
}
