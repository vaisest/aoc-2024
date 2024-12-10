use itertools::{enumerate, Itertools};

enum Space {
    File(usize),
    Free,
}
fn scan_to_next_free(vec: &Vec<Space>, mut it: usize) -> Option<usize> {
    loop {
        match vec[it] {
            Space::File(_) => {
                it += 1;
            }
            Space::Free => {
                break;
            }
        }
        if it >= vec.len() {
            return None;
        }
    }
    Some(it)
}
fn prune_vec(vec: &mut Vec<Space>) {
    // prune right side of empty space
    loop {
        match vec.last().unwrap() {
            Space::Free => {
                vec.pop();
            }
            Space::File(_) => {
                break;
            }
        }
    }
}
pub fn part1(input: String) -> String {
    let mut vec = Vec::new();
    input
        .chars()
        .filter(|c| c.is_digit(10))
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            (
                // this should never fail
                chunk.next().unwrap().to_digit(10).unwrap(),
                // this can be None on end of input
                chunk.next().unwrap_or('0').to_digit(10).unwrap(),
            )
        })
        .enumerate()
        .for_each(|(file_id, (file_len, free_len))| {
            for _ in 0..file_len {
                vec.push(Space::File(file_id))
            }
            for _ in 0..free_len {
                vec.push(Space::Free);
            }
        });

    prune_vec(&mut vec);

    // move rightmost file block to the leftmost free block
    let mut left = Some(0);
    while left.is_some() {
        left = scan_to_next_free(&vec, left.unwrap());
        match left {
            Some(i) => {
                let last_block = vec.pop().unwrap();
                vec[i] = last_block
            }
            None => {
                break;
            }
        };
    }

    // checksum
    vec.into_iter()
        .enumerate()
        .map(|(pos, space)| match space {
            Space::File(id) => pos * id,
            Space::Free => panic!("there should be no free space here"),
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    "-1".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_p1() {
        let input = "2333133121414131402".to_string();
        assert_eq!(part1(input), "1928");
    }

    #[test]
    fn sample_p2() {
        let input = "".to_string();
        assert_eq!(part2(input), "31");
    }
}
