use std::u32;

pub fn part1(input: String) -> String {
    let safe = input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|split| {
            let mut decrement: Option<u32> = None;
            split.reduce(|left, right| {
                let decrease = left.parse::<u32>().unwrap() - right.parse::<u32>().unwrap();
            })
        });

    String::new()
}

pub fn part2(input: String) -> String {
    String::new()
}

mod tests {
    use super::*;

    #[test]
    fn sample_p1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 "
            .to_string();
        assert_eq!(part1(input), "2");
    }

    // #[test]
    // fn sample_p2() {
    //     let input = "".to_string();
    //     assert_eq!(part2(input), "31");
    // }
}
