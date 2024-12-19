pub fn part1(input: String) -> String {
    "-1".to_string()
}

pub fn part2(input: String) -> String {
    "-1".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = "".to_string();
        assert_eq!(part1(input), "11");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = "".to_string();
        assert_eq!(part2(input), "31");
    }
}
