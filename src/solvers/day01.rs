use std::collections::HashMap;

pub fn part1(input: String) -> String {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    for line in input.lines().filter(|x| !x.is_empty()) {
        let (l, r) = line
            .split_once(" ")
            .unwrap_or_else(|| panic!("could not split: {line}"));

        left.push(l.parse::<u32>().unwrap());
        right.push(
            // split_once leaves whitespace for some reason
            r.trim_start().parse::<u32>().unwrap(),
        );
    }

    left.sort();
    right.sort();

    assert_eq!(left.len(), right.len());

    let mut sum = 0u32;
    for (l, r) in left.iter().zip(right) {
        sum += l.abs_diff(r);
    }

    sum.to_string()
}

pub fn part2(input: String) -> String {
    let mut left: Vec<u32> = vec![];
    let mut right = HashMap::<u32, u32>::new();

    for line in input.lines().filter(|x| !x.is_empty()) {
        let (l, r) = line
            .split_once(" ")
            .unwrap_or_else(|| panic!("could not split: {line}"));

        left.push(l.parse::<u32>().unwrap());
        *right
            .entry(r.trim_start().parse::<u32>().unwrap())
            .or_insert(0) += 1;
    }

    left.iter()
        .map(|it| it * right.get(it).unwrap_or(&0))
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_p1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3"
            .to_string();
        assert_eq!(part1(input), "11");
    }

    #[test]
    fn sample_p2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3"
            .to_string();
        assert_eq!(part2(input), "31");
    }
}
