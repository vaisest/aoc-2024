use std::{cmp::Ordering, collections::BTreeSet};

use arrayvec::ArrayVec;
fn parse_input(input: String) -> (BTreeSet<(u32, u32)>, Vec<ArrayVec<u32, 24>>) {
    let (rules, pages) = input.split_once("\n\n").unwrap();

    let rule_pairs = rules
        .lines()
        .filter_map(|line| line.split_once("|"))
        .map(|(one, two)| (one.parse::<u32>().unwrap(), two.parse::<u32>().unwrap()))
        .collect::<BTreeSet<(u32, u32)>>();

    let updates = pages
        .lines()
        .map(|line| {
            line.split(",")
                .map(|it| it.parse::<u32>().unwrap())
                .collect::<ArrayVec<u32, 24>>()
        })
        .collect::<Vec<ArrayVec<u32, 24>>>();
    (rule_pairs, updates)
}

fn comp(lhs: u32, rhs: u32, rule_pairs: &BTreeSet<(u32, u32)>) -> bool {
    // interestingly, the list of rule pairs seems to be very complete.
    // it seems that it's enough to compare single pairs,
    // even though the opposite could be expected
    if rule_pairs.contains(&(lhs, rhs)) {
        return true;
    }
    return false;
}
fn ord_comp(lhs: u32, rhs: u32, rule_pairs: &BTreeSet<(u32, u32)>) -> Ordering {
    if rule_pairs.contains(&(lhs, rhs)) {
        return Ordering::Less;
    }
    return Ordering::Equal;
}
pub fn part1(input: String) -> String {
    // this problem seems like a topological sort problem, but it seems the rule pair list is
    // a cyclic graph on its own, even though a set of rules that apply to a single update is
    // acyclic.
    // so instead of doing that, we order using the rule list
    let (rule_pairs, updates) = parse_input(input);

    updates
        .iter()
        // filter to correct updates
        .filter(|update| update.is_sorted_by(|&lhs, &rhs| comp(lhs, rhs, &rule_pairs)))
        // return sum of medians
        .map(|arr| arr[arr.len() / 2])
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let (rule_pairs, mut updates) = parse_input(input);

    updates
        .iter_mut()
        // filter to incorrect updates
        .filter(|update| !update.is_sorted_by(|&lhs, &rhs| comp(lhs, rhs, &rule_pairs)))
        // correct the incorrect updates
        .map(|update| {
            update.sort_by(|lhs, rhs| ord_comp(*lhs, *rhs, &rule_pairs));
            update
        })
        // and return sum of medians
        .map(|arr| arr[arr.len() / 2])
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_p1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            .to_string();
        assert_eq!(part1(input), "143");
    }

    #[test]
    fn sample_p2() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            .to_string();
        assert_eq!(part2(input), "123");
    }
}
