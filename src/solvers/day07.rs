use std::u64;

fn check_possible_equivalence_p1(desired_result: u64, mut operands: Vec<u64>) -> Option<u64> {
    let mut output = Vec::new();
    // used for temporary storage of results while iterating through the output vec
    let mut secondary = Vec::new();
    // we get n^3 results, so we can reserve that much space on a result vector
    output.reserve_exact(operands.len().pow(2));
    // we have two identical sized arrays for swapping
    secondary.reserve_exact(operands.len().pow(2));

    // initialise output array separately since only the first operation operates on two literals
    let first = operands.pop().unwrap();
    let second = operands.pop().unwrap();
    output.extend([first + second, first * second]);

    while !operands.is_empty() {
        let rhs = operands.pop().unwrap();

        for &num in output.iter() {
            let triple = [num + rhs, num * rhs];
            // we could short-circuit out if we notice that all results are larger than
            // the desired result, but it seems the overhead of that is more costly
            secondary.extend(triple);
        }
        std::mem::swap(&mut output, &mut secondary);
        secondary.clear();
    }

    output.into_iter().find(|&it| it == desired_result)
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = (u64, Vec<u64>)> + 'a {
    input.lines().map(|line| {
        // split expected result and operands
        let (res, eq) = line.split_once(":").unwrap();
        let eq_numbers = eq
            // split_once seems to leave trailing whitespace: trim it
            .trim_ascii_start()
            .split_ascii_whitespace()
            .map(|word| word.parse().unwrap())
            // reverse to parse from left to right
            .rev()
            .collect::<Vec<u64>>();
        (res.parse().unwrap(), eq_numbers)
    })
}

pub fn part1(input: String) -> String {
    parse_input(&input)
        .filter_map(|(desired_result, operands)| {
            check_possible_equivalence_p1(desired_result, operands)
        })
        .sum::<u64>()
        .to_string()
}

fn u64_concatenate(lhs: u64, rhs: u64) -> u64 {
    // concatenate by determining number of zeroes in rhs
    // and moving lhs left by that many digits
    let lhs_digits = rhs.ilog10() + 1;
    lhs * 10u64.pow(lhs_digits) + rhs
}

fn check_possible_equivalence_p2(desired_result: u64, mut operands: Vec<u64>) -> Option<u64> {
    // nearly the same as p1, just with 3 results instead of 2 results per number
    let mut output = Vec::new();
    // used for temporary storage of results while iterating through the output vec
    let mut secondary = Vec::new();
    // we get n^3 results, so we can reserve that much space on a result vector
    output.reserve_exact(operands.len().pow(3));
    // we have two identical sized arrays for swapping
    secondary.reserve_exact(operands.len().pow(3));

    // initialise output array separately since only the first operation operates on two literals
    let first = operands.pop().unwrap();
    let second = operands.pop().unwrap();
    output.extend([
        first + second,
        first * second,
        u64_concatenate(first, second),
    ]);

    while !operands.is_empty() {
        let rhs = operands.pop().unwrap();

        for &num in output.iter() {
            let triple = [num + rhs, num * rhs, u64_concatenate(num, rhs)];
            // we could short-circuit out if we notice that all results are larger than
            // the desired result, but it seems the overhead of that is more costly
            secondary.extend(triple);
        }
        std::mem::swap(&mut output, &mut secondary);
        secondary.clear();
    }

    output.into_iter().find(|&it| it == desired_result)
}

pub fn part2(input: String) -> String {
    parse_input(&input)
        .filter_map(|(desired_result, operands)| {
            check_possible_equivalence_p2(desired_result, operands)
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

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
