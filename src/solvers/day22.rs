// fn mix(lhs: u64, rhs: u64) -> u64 {}

use rustc_hash::{FxHashMap, FxHashSet};

fn process(mut monkey: u64) -> u64 {
    monkey = ((64 * monkey) ^ monkey) % 16777216;

    monkey = (monkey ^ (monkey / 32)) % 16777216;

    monkey = ((monkey * 2048) ^ monkey) % 16777216;

    monkey
}

pub fn part1(input: String) -> String {
    let start = std::time::Instant::now();
    let monkeys = input.lines().map(|line| line.parse::<u64>().unwrap());

    let mut total = 0;
    for mut monkey in monkeys {
        for _ in 0..2000 {
            monkey = process(monkey);
        }
        total += monkey;
    }
    println!("elapsed millis: {}", start.elapsed().as_millis());
    total.to_string()
}

fn index(deltas: (i8, i8, i8, i8)) -> usize {
    (deltas.0 + 9) as usize * 19usize.pow(3)
        + (deltas.1 + 9) as usize * 19usize.pow(2)
        + (deltas.2 + 9) as usize * 19usize
        + (deltas.3 + 9) as usize
}

pub fn part2(input: String) -> String {
    let start = std::time::Instant::now();
    let monkeys = input.lines().map(|line| line.parse::<u64>().unwrap());

    // map from 4 deltas to the total amount of bananas it buys
    // let mut map: FxHashMap<_, u64> = FxHashMap::default();
    // somehow this massive array is faster than the hashmap
    let mut map = vec![0; 19usize.pow(4)];
    for mut monkey in monkeys {
        // we want to avoid checking delta sequences multiple times as the
        // monkey buys the first one that matches
        let mut seen = FxHashSet::default();
        let mut old_price = (monkey % 10) as i8;
        let mut deltas = vec![];

        for _ in 0..2000 {
            monkey = process(monkey);
            let price = (monkey % 10) as i8;

            let delta = (price as i64 - old_price as i64) as i8;
            deltas.push(delta);

            old_price = price;

            let n = deltas.len();
            if n < 4 {
                continue;
            }
            let deltas = (deltas[n - 4], deltas[n - 3], deltas[n - 2], deltas[n - 1]);

            if seen.contains(&deltas) {
                continue;
            }
            seen.insert(deltas);
            map[index(deltas)] += price;
        }
    }
    // let best = *map.values().max().unwrap();
    let best = map.into_iter().max().unwrap();

    println!("elapsed millis: {}", start.elapsed().as_millis());
    best.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = "1
10
100
2024"
            .to_string();
        assert_eq!(part1(input), "37327623");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = "1
2
3
2024"
            .to_string();
        assert_eq!(part2(input), "23");
    }
}
