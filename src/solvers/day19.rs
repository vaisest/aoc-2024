use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashMap;

// a homebrew and much slower version of the regex
// fn can_make_design(original_design: &str, patterns: &Vec<&str>) -> bool {
//     let mut to_test = vec![original_design];
//     let mut seen = FxHashSet::default();
//     while let Some(design) = to_test.pop() {
//         // // avoid testing the same patterns again
//         if seen.contains(design) {
//             continue;
//         }
//         seen.insert(design);
//         if design == "" {
//             return true;
//         }
//         for pattern in patterns.iter() {
//             if pattern.len() > design.len() {
//                 break;
//             }

//             if design.starts_with(pattern) {
//                 to_test.push(&design[pattern.len()..]);
//             }
//         }
//     }
//     false
// }

pub fn part1(input: String) -> String {
    let (patterns, designs) = input.split_once("\n\n").unwrap();

    let pattern_re_string = format!("^({})+$", patterns.split(", ").join("|"));
    let pattern_re =
        Regex::new(&pattern_re_string).expect("could not prepare regex. is the data malformed?");

    designs
        .lines()
        .filter(|design| pattern_re.is_match(design))
        .count()
        .to_string()
}

fn try_p2<'a>(design: &'a str, patterns: &Vec<&str>, cache: &mut FxHashMap<&'a str, u64>) -> u64 {
    // memoization
    if let Some(&res) = cache.get(&design) {
        return res;
    }
    let mut count = 0;
    // base case
    if design == "" {
        return 1;
    }

    for pattern in patterns {
        // designs are sorted so we can break early when they get too long
        if pattern.len() > design.len() {
            break;
        }
        // recurse with all matching patterns removed from the beginning
        if design.starts_with(pattern) {
            count += try_p2(&design[pattern.len()..], patterns, cache);
        }
    }
    cache.insert(design, count);
    count
}

pub fn part2(input: String) -> String {
    let (patterns, designs) = input.split_once("\n\n").unwrap();

    let mut patterns = patterns.split(", ").collect::<Vec<_>>();
    patterns.sort_by_key(|s| s.len());

    let mut cache = FxHashMap::default();

    designs
        .lines()
        .map(|design| try_p2(design, &patterns, &mut cache))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
            .to_string();
        assert_eq!(part1(input), "6");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
            .to_string();
        assert_eq!(part2(input), "16");
    }
}
