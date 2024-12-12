use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    usize,
};

use itertools::Itertools;

#[derive(PartialEq, Eq)]
enum Space {
    File(usize),
    Free,
}
impl Debug for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Space::File(a) => write!(f, "{}", a),
            &Space::Free => write!(f, "."),
        }
    }
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
            Space::Free => 0,
        })
        .sum::<usize>()
        .to_string()
}

fn try_move(
    vec: &mut Vec<Space>,
    buf: &Vec<usize>,
    space_spans: &mut BTreeMap<usize, BTreeSet<usize>>,
    from_index: usize,
) -> bool {
    // flush buffer into first contiguous free fitting span of blocks

    assert!(buf.len() != 0);

    // find first free slot that's to the left of our source
    let span = space_spans
        .range_mut(buf.len()..)
        .filter(|(_, set)| *set.first().unwrap() < from_index)
        .sorted_by_key(|(_, set)| *set.first().unwrap())
        .next();
    if span.is_none() {
        return false;
    }

    // we get a known span of free space and overwrite it from the start
    let (&len, idx_set) = span.unwrap();
    let idx = idx_set.pop_first().unwrap();
    let mut pointer = idx;
    for &file_id in buf.iter().rev() {
        vec[pointer] = Space::File(file_id);
        pointer += 1;
    }

    // resize or remove the free span
    if idx_set.len() == 0 {
        space_spans.remove(&len);
    }
    if buf.len() != len {
        space_spans
            .entry(len - buf.len())
            .or_default()
            .insert(idx + buf.len());
    }

    for idx in 0..buf.len() {
        vec[from_index + idx] = Space::Free;
    }

    true
}

pub fn part2(input: String) -> String {
    let mut vec = Vec::new();
    // map could be an array as lengths seem to be limited to 0-9
    let mut space_spans: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    // assert!(input.len().rem(2) == 1);
    input
        .chars()
        .filter(|c| c.is_digit(10))
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            (
                // this should never fail,
                chunk.next().unwrap().to_digit(10).unwrap(),
                // but this can be None on end of input
                chunk.next().unwrap_or('0').to_digit(10).unwrap(),
            )
        })
        .enumerate()
        .for_each(|(file_id, (file_len, free_len))| {
            for _ in 0..file_len {
                vec.push(Space::File(file_id))
            }
            if free_len != 0 {
                // save start index of the free slot
                space_spans
                    .entry(free_len as usize)
                    .or_default()
                    .insert(vec.len());
            }
            for _ in 0..free_len {
                vec.push(Space::Free);
            }
        });

    // move rightmost file block to the leftmost free block
    let mut lowest_id_processed = usize::MAX;
    let mut buf = Vec::new();
    for right in (0..vec.len()).rev() {
        match vec[right] {
            // if buffer isn't empty and we hit a free block, try move
            Space::Free => {
                if buf.len() != 0 {
                    try_move(&mut vec, &buf, &mut space_spans, right + 1);
                    lowest_id_processed = *buf.last().unwrap();
                    buf.clear();
                }
            }
            Space::File(id) => {
                // if buffer isn't empty and we hit a different id file block, try move
                if buf.last().is_some_and(|it| *it != id) {
                    try_move(&mut vec, &buf, &mut space_spans, right + 1);
                    lowest_id_processed = *buf.last().unwrap();
                    buf.clear();
                }
                // avoid moving files twice
                if id < lowest_id_processed {
                    buf.push(id);
                }
            }
        }
    }

    // checksum
    vec.into_iter()
        .enumerate()
        .map(|(pos, space)| match space {
            Space::File(id) => pos * id,
            Space::Free => 0,
        })
        .sum::<usize>()
        .to_string()
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
        let input = "12101".to_string();
        assert_eq!(part2(input), "4");

        let input = "714892711".to_string();
        assert_eq!(part2(input), "813");

        let input = "2333133121414131499".to_string();
        assert_eq!(part2(input), "6204");

        let input = "1313165".to_string();
        assert_eq!(part2(input), "169");

        let input = "0112233".to_string();
        assert_eq!(part2(input), "73");

        let input = "2333133121414131402".to_string();
        assert_eq!(part2(input), "2858");

        let input = "23222".to_string();
        assert_eq!(part2(input), "21");
    }
}
