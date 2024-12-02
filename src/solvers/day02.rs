fn verify_sequence_iter(mut sequence: impl Iterator<Item = i32>) -> bool {
    let mut increasing: Option<bool> = None;
    let mut last = sequence.next().unwrap();

    for number in sequence {
        let diff = last - number;
        if increasing.is_none() {
            increasing.replace(if diff < 0 { false } else { true });
        }
        // valid conditions: difference between each 1 < x < 3
        if (diff == 0 || diff.abs() > 3)
            // and monotonically increasing or decreasing
            || (diff < 0 && increasing.unwrap())
            || (diff > 0 && !increasing.unwrap())
        {
            return false;
        }
        last = number;
    }
    true
}

pub fn part1(input: String) -> String {
    input
        .lines()
        .filter_map(|line| {
            let iterator = line
                .split_whitespace()
                .map(|it| it.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
                .into_iter();

            if verify_sequence_iter(iterator) {
                Some(line)
            } else {
                None
            }
        })
        .count()
        .to_string()
}

pub fn part2(input: String) -> String {
    // same as p1 but we can drop up to 1 number
    // from the sequence to fix it
    input
        .lines()
        .filter_map(|line| {
            let vec = line
                .split_whitespace()
                .map(|it| it.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            for i in 0..vec.len() {
                // drop ith number and verify that slice.
                // there seem to only be up to 6 numbers per
                // line which makes this not-too-expensive
                let sequence =
                    vec.iter()
                        .enumerate()
                        .filter_map(|(j, x)| if i != j { Some(*x) } else { None });

                if verify_sequence_iter(sequence) {
                    return Some(line);
                }
            }
            None
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_p1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .to_string();
        assert_eq!(part1(input), "2");
    }

    #[test]
    fn sample_p2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .to_string();
        assert_eq!(part2(input), "4");
    }
}
