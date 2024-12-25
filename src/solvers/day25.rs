pub fn part1(input: String) -> String {
    let mut locks = vec![];
    let mut keys = vec![];
    input.split("\n\n").for_each(|block| {
        let mut bits = 0u64;
        block
            .lines()
            .map(|line| line.chars())
            .flatten()
            .enumerate()
            .for_each(|(i, c)| match c {
                // set i-th bit
                '#' => bits |= 1 << i,
                _ => {}
            });
        if bits & 1 == 1 {
            locks.push(bits);
        } else {
            keys.push(bits);
        }
    });

    let mut total = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            // NAND -> if any overlaps, result is not zero
            if lock & key == 0 {
                total += 1;
            }
        }
    }

    total.to_string()
}

pub fn part2(_input: String) -> String {
    "There was no day 25 part 2".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"
            .to_string();
        assert_eq!(part1(input), "3");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = "".to_string();
        assert_eq!(part2(input), "There was no day 25 part 2");
    }
}
