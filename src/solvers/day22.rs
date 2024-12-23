use std::array;

use bitvec::bitvec;

fn process(mut monkey: u64) -> u64 {
    monkey = ((64 * monkey) ^ monkey) % 16777216;

    monkey = (monkey ^ (monkey / 32)) % 16777216;

    monkey = ((monkey * 2048) ^ monkey) % 16777216;

    monkey
}

pub fn part1(input: String) -> String {
    let monkeys = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut total = 0;
    const LANES: usize = 16;
    for monkes in monkeys.chunks_exact(LANES) {
        let mut monkes: [u64; LANES] = array::from_fn(|i| monkes[i]);
        for _ in 0..2000 {
            monkes = monkes
                .map(|monkey| ((64 * monkey) ^ monkey) % 16777216)
                .map(|monkey| (monkey ^ (monkey / 32)) % 16777216)
                .map(|monkey| ((monkey * 2048) ^ monkey) % 16777216);
        }
        for monkey in monkes {
            total += monkey;
        }
    }
    for monkey in monkeys.chunks_exact(LANES).remainder() {
        let mut monkey = *monkey;
        for _ in 0..2000 {
            monkey = process(monkey);
        }
        total += monkey;
    }
    total.to_string()
}

fn index(deltas: (i8, i8, i8, i8)) -> usize {
    (deltas.0 + 9) as usize * 19usize.pow(3)
        + (deltas.1 + 9) as usize * 19usize.pow(2)
        + (deltas.2 + 9) as usize * 19usize
        + (deltas.3 + 9) as usize
}

pub fn part2(input: String) -> String {
    let monkeys = input.lines().map(|line| line.parse::<u64>().unwrap());

    // map from 4 deltas to the total amount of bananas it buys
    // we use vectors with indexes based on the delta sequenes, as while
    // there are a lot of possible combinations, the total amount
    // is still relatively low and clearly outperforms fxhashmap
    let mut map = vec![0; 19usize.pow(4)];
    for mut monkey in monkeys {
        // we want to avoid checking delta sequences multiple times as the
        // monkey buys the first one that matches
        let mut seen = bitvec![0; 19usize.pow(4)];
        let mut old_price = (monkey % 10) as i8;
        let mut deltas = vec![];

        for _ in 0..2000 {
            monkey = process(monkey);
            let price = (monkey % 10) as i8;

            let delta = price - old_price;
            deltas.push(delta);

            old_price = price;

            let n = deltas.len();
            if n < 4 {
                continue;
            }
            let deltas = (deltas[n - 4], deltas[n - 3], deltas[n - 2], deltas[n - 1]);

            let idx = index(deltas);
            if seen[idx] {
                continue;
            }
            seen.set(idx, true);
            map[index(deltas)] += price as i16;
        }
    }

    map.into_iter().max().unwrap().to_string()
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
