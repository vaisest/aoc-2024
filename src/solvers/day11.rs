use rustc_hash::FxHashMap;

fn split_number_digitwise(n: u64, digit_count: u32) -> (u64, u64) {
    let pow = 10u64.pow(digit_count / 2);
    let left = n / pow;
    let right = n - left * pow;
    (left, right)
}

pub fn part1(input: String) -> String {
    let stones = input
        .split_whitespace()
        .map(|word| word.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    const N_ITER: u32 = 25;
    iter_each(stones, N_ITER).to_string()
}

fn iter_each(stones: Vec<u64>, max_iter: u32) -> u64 {
    let mut memo = FxHashMap::default();

    let mut count = 0u64;
    for origin_stone in stones.iter() {
        count += iterate_single_stone(*origin_stone, max_iter, &mut memo, 0);
        memo.clear();
    }
    count
}

fn iterate_single_stone(
    stone: u64,
    iter_max: u32,
    mut map: &mut FxHashMap<(u64, u32), u64>,
    // should start from zero
    iteration: u32,
) -> u64 {
    // recursion with memoization. We save (stone, iteration) pairs
    // that map to the amount of stones it produces at iteration iter_max
    if iteration == iter_max {
        return 1;
    }

    if let Some(&early_res) = map.get(&(stone, iteration)) {
        return early_res;
    }

    let res;
    if stone == 0 {
        // Stone is 0 => stone replaced by 1
        res = iterate_single_stone(1, iter_max, &mut map, iteration + 1);
    } else if (stone.ilog10() + 1) % 2 == 0 {
        // Stone digit count is even => Split in two, left half of digits on left one, right on right.
        let (l, r) = split_number_digitwise(stone, stone.ilog10() + 1);

        res = iterate_single_stone(l, iter_max, &mut map, iteration + 1)
            + iterate_single_stone(r, iter_max, &mut map, iteration + 1);
    } else {
        // Otherwise => stone replace by multiplying it by 2024
        res = iterate_single_stone(stone * 2024, iter_max, &mut map, iteration + 1);
    }
    map.insert((stone, iteration), res);
    res
}

pub fn part2(input: String) -> String {
    let stones = input
        .split_whitespace()
        .map(|word| word.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    const N_ITER: u32 = 75;
    iter_each(stones, N_ITER).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_p1() {
        let input = "125 17".to_string();
        assert_eq!(part1(input), "55312");

        let input = "125".to_string();
        assert_eq!(part1(input), "19025");
    }

    #[test]
    fn sample_p2() {
        let input = "125 17".to_string();
        assert_eq!(part2(input), "65601038650482");

        let input = "125".to_string();
        assert_eq!(part2(input), "22840618691206");
    }
}
