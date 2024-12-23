use rustc_hash::{FxHashMap, FxHashSet};
fn parse_input(input: &str) -> (FxHashMap<&str, FxHashSet<&str>>, Vec<&str>, usize) {
    let mappings = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect::<Vec<(&str, &str)>>();
    let mut map: FxHashMap<&str, FxHashSet<_>> = FxHashMap::default();
    for &(one, two) in mappings.iter() {
        map.entry(one)
            .and_modify(|set| {
                set.insert(two);
            })
            .or_insert_with(|| {
                let mut set = FxHashSet::default();
                set.insert(two);
                set
            });
        map.entry(two)
            .and_modify(|set| {
                set.insert(one);
            })
            .or_insert_with(|| {
                let mut set = FxHashSet::default();
                set.insert(one);
                set
            });
    }
    let t_computers = map
        .keys()
        .filter(|str| str.starts_with('t'))
        .map(|&it| it)
        .collect::<Vec<_>>();
    let t_degree = t_computers
        .iter()
        .map(|s| map.get(s).unwrap().len())
        .max()
        .unwrap();
    (map, t_computers, t_degree)
}
fn find_cliques_from(
    start: &str,
    map: &FxHashMap<&str, FxHashSet<&str>>,
    // these are shared between the start strings, so we handle them separately
    multi_t: &mut FxHashSet<String>,
) -> usize {
    let mut stack = vec![vec![start]];
    let mut seen = FxHashSet::default();
    let mut total = 0usize;
    while let Some(mut current) = stack.pop() {
        // we check for seen sequences by checking that the latter two (that
        // don't start from a t) do not exist yet
        let mut hash = ["", ""];
        let mut hash2 = ["", ""];
        let slice = &current[1..];
        assert!(slice.len() <= 2);
        for (i, elem) in slice.iter().enumerate() {
            hash[i] = elem;
            hash2[1 - i] = elem;
        }
        if seen.contains(&hash) || seen.contains(&hash2) {
            continue;
        }
        seen.insert(hash);

        if current.len() == 3 {
            // the special case of having a multi-t sequence
            if current[1].starts_with('t') || current[2].starts_with('t') {
                current.sort();
                multi_t.insert(current.join(""));
            } else {
                total += 1;
            }

            continue;
        }
        // count how many nodes share a neighbour
        let mut both: FxHashMap<&str, usize> = FxHashMap::default();
        for current_node in current.iter() {
            if let Some(destinations) = map.get(current_node) {
                for &dest in destinations {
                    both.entry(dest).and_modify(|it| *it += 1).or_insert(1);
                }
            }
        }

        for (&dest, &count) in both.iter() {
            // if both share a neighbour, push it to the stack
            if count == current.len() {
                let mut new = current.clone();
                new.push(dest);
                stack.push(new);
            }
        }
    }
    total
}
pub fn part1(input: String) -> String {
    let (map, t_computers, _) = parse_input(&input);
    let mut output2 = FxHashSet::default();
    let count = t_computers
        .into_iter()
        .map(|start| find_cliques_from(start, &map, &mut output2))
        .sum::<usize>();
    (count + output2.len()).to_string()
}

fn find_maxmimum_clique_from(
    start: &str,
    map: &FxHashMap<&str, FxHashSet<&str>>,
    k: usize,
) -> Option<String> {
    let mut stack = vec![vec![start]];
    let mut seen = FxHashSet::default();
    while let Some(mut current) = stack.pop() {
        if current.len() == k {
            current.sort();
            return Some(current.join(","));
        }
        // there may be duplicates that are discovered in a different order, and
        // thus we have to check each element
        if current.iter().all(|&s| seen.contains(s)) {
            continue;
        }
        seen.insert(current.last().unwrap().to_string());

        // how many times a neighbour is shared between nodes in current
        let mut new_clique_vertices: FxHashMap<&str, usize> = FxHashMap::default();
        for node in current.iter() {
            let adjacents = map.get(node).unwrap();
            for &dest in adjacents {
                new_clique_vertices
                    .entry(dest)
                    .and_modify(|it| *it += 1)
                    .or_insert(1);
            }
        }
        // each node in current shares a neighbour, it's a clique candidate and
        // can be added to the stack
        let possible_members = new_clique_vertices
            .into_iter()
            .filter(|&(_, n)| n == current.len());
        for (adj, _) in possible_members {
            let mut new = current.clone();
            new.push(adj);
            stack.push(new);
        }
    }
    None
}
pub fn part2(input: String) -> String {
    let (map, t_computers, degree) = parse_input(&input);
    // At least for my input, the largest input contains a t-node. I'm guessing
    // here that this is true for all inputs as a reference to part 1. However,
    // if it isn't, this solution is incorrect and would need to be checked with
    // many more start nodes.

    for i in (0..=degree).rev() {
        for start in t_computers.iter() {
            if let Some(res) = find_maxmimum_clique_from(start, &map, i) {
                return res;
            }
        }
    }

    "no answer found".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"
            .to_string();
        assert_eq!(part1(input), "7");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"
            .to_string();
        assert_eq!(part2(input), "co,de,ka,ta");
    }
}
