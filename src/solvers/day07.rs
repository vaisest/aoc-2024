fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = (u64, Vec<u64>)> + 'a {
    input.lines().map(|line| {
        // split expected result and operands
        let (res, eq) = line.split_once(":").unwrap();
        let eq_numbers = eq
            // split_once seems to leave trailing whitespace: trim it
            .trim_ascii_start()
            .split_ascii_whitespace()
            .map(|word| word.parse().unwrap())
            // // reverse to parse from left to right
            // .rev()
            .collect::<Vec<u64>>();
        (res.parse().unwrap(), eq_numbers)
    })
}

fn check(desired_result: u64, op_slice: &[u64], try_concatenation: bool) -> bool {
    match op_slice {
        [operand] => *operand == desired_result,
        [head @ .., operand] => {
            // if the end result is divisible, last might be a part of a multiplier operation
            (desired_result % operand == 0 && check(desired_result / operand, head, try_concatenation))
            // and concatenation is only possible if the last digits are equal to it
            || (try_concatenation && is_suffix(desired_result, *operand) && check(desired_result / 10u64.pow(operand.ilog10()+1), head, try_concatenation))
            // addition is almost always possible so we leave it for the last
            || (desired_result > *operand && check(desired_result - operand, head, try_concatenation))
        }
        _ => unreachable!(),
    }
}

pub fn part1(input: String) -> String {
    parse_input(&input)
        .filter_map(|(desired_result, operands)| {
            if check(desired_result, &operands[..], false) {
                Some(desired_result)
            } else {
                None
            }
        })
        .sum::<u64>()
        .to_string()
}

fn is_suffix(big: u64, small: u64) -> bool {
    // big-small should make the result have zeros at the end, so if we only
    // take the same last digits as small has, we should get zero if it is a
    // suffix
    let small_digits = small.ilog10() + 1;
    (big - small) % 10u64.pow(small_digits) == 0
}

pub fn part2(input: String) -> String {
    parse_input(&input)
        .filter_map(|(desired_result, operands)| {
            if check(desired_result, &operands[..], true) {
                Some(desired_result)
            } else {
                None
            }
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix() {
        let rhs = 3034;
        let rhs2 = 3333;
        let lhs = 86987563034;
        assert!(is_suffix(lhs, rhs));
        assert!(!is_suffix(lhs, rhs2));
    }

    #[test]
    fn sample_p1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            .to_string();
        assert_eq!(part1(input), "3749");
    }

    #[test]
    fn sample_p2() {
        let input = "156: 15 6
"
        .to_string();
        assert_eq!(part2(input), "156");

        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            .to_string();
        assert_eq!(part2(input), "11387");
    }
}
