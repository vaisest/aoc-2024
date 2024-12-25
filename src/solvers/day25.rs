fn get_height(lock: &Vec<Vec<char>>, is_key: bool) -> [usize; 5] {
    let pins = lock[0].len();
    let mut pin_heights = [None; 5];
    if is_key {
        for x in 0..pins {
            for y in (0..lock.len()).rev() {
                if pin_heights[x].is_none() && lock[y][x] != '#' {
                    pin_heights[x].replace(lock.len() - y - 1);
                }
            }
        }
    } else {
        for x in 0..pins {
            for y in 0..lock.len() {
                if pin_heights[x].is_none() && lock[y][x] != '#' {
                    pin_heights[x].replace(y);
                }
            }
        }
    }
    pin_heights.map(|e| e.unwrap())
}
fn fits((lock, lock_height): &([usize; 5], usize), key: &[usize; 5]) -> bool {
    lock.into_iter()
        .zip(key.into_iter())
        .all(|(&lock_h, &key_h)| lock_h + key_h <= *lock_height)
}

pub fn part1(input: String) -> String {
    let mut locks = vec![];
    let mut keys = vec![];
    input.split("\n\n").for_each(|block| {
        let v = block
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<_>>>();
        if v[0][0] == '#' {
            locks.push((get_height(&v, false), v.len()));
        } else {
            keys.push(get_height(&v, true));
        }
    });

    let mut total = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if fits(lock, key) {
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
        assert_eq!(part2(input), "31");
    }
}
