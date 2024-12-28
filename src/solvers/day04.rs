use arrayvec::ArrayVec;

fn try_apply_direction<'a>(
    y: usize,
    x: usize,
    (dy, dx): (i16, i16),
    arr: &Vec<&'a [u8]>,
    mul: i16,
) -> Option<&'a u8> {
    let ny = (y as i16 + dy * mul) as usize;
    let nx = (x as i16 + dx * mul) as usize;
    arr.get(ny).and_then(|it| it.get(nx))
}

// fn char_lines(input: String) -> impl Iterator<Item = Vec<u8>> {
fn char_lines(bytes: &Vec<u8>) -> impl Iterator<Item = &[u8]> {
    bytes
        .split(|c| {
            if c.is_ascii() {
                *c == b'\n'
            } else {
                panic!("input is not ascii. is it not a regular aoc input file?")
            }
        })
        .filter(|line| line.len() != 0)
}

pub fn part1(input: String) -> String {
    let u8s = input.into_bytes();
    let mat = char_lines(&u8s)
        // .map(|line| line.to_vec())
        .collect::<Vec<_>>();

    // ensure input is square
    assert!(mat.iter().all(|line| line.len() == mat.len()));

    let mut count = 0u32;
    for y in 0..mat.len() {
        for x in 0..mat.len() {
            // (dy, dx)
            let directions = [
                (0, 1),  // R
                (1, 1),  // BR
                (-1, 1), // TR
                (1, 0),  // D
            ];

            // we only check the start points of words
            // and know based on that character which one we're looking for
            let word = match mat[y][x] {
                b'X' => [b'X', b'M', b'A', b'S'],
                b'S' => [b'S', b'A', b'M', b'X'],
                _ => continue,
            };

            'dir: for dir in directions.iter() {
                for mul in (1..=3).rev() {
                    // check the direction scaled by mul and see if we get something or are oob
                    let x = try_apply_direction(y, x, *dir, &mat, mul);
                    // if none, x is oob. it can also just not be the right character we're looking for
                    if x.is_none() || *x.unwrap() != word[mul as usize] {
                        continue 'dir;
                    }
                }
                count += 1;
            }
        }
    }
    count.to_string()
}

// fn add_diag(coord: usize, diag: i16) -> usize
pub fn part2(input: String) -> String {
    let u8s = input.into_bytes();
    let mat = char_lines(&u8s)
        // .map(|line| line.to_vec())
        .collect::<Vec<_>>();

    // ensure input is square
    assert!(mat.iter().all(|line| line.len() == mat.len()));

    let mut count = 0;
    for y in 0..mat.len() {
        for x in 0..mat.len() {
            // we only want to check the middle of the X-MAS
            // as we're doing it by checking the diagonals
            if mat[y][x] != b'A' {
                continue;
            }

            // (dy, dx)
            let diags = [
                (-1, -1), // TL
                (1, 1),   // BR
                (1, -1),  // BL
                (-1, 1),  // TR
            ];

            let chars = diags
                .iter()
                .filter_map(|(dy, dx)| {
                    mat.get((y as i16 + dy) as usize)
                        .and_then(|line| line.get((x as i16 + dx) as usize).copied())
                })
                .collect::<ArrayVec<u8, 4>>();
            // if some were out of bounds, there won't be enough and
            // it can't be an X-MAS
            if chars.len() < 4 {
                continue;
            }

            // first two are diagonal from TL to BR, and the latter two are diagonal from BL to TR
            let tl = chars[0];
            let br = chars[1];
            let bl = chars[2];
            let tr = chars[3];
            // check if we got MAS or SAM in both
            if ((tl == b'M' && br == b'S') || (tl == b'S' && br == b'M'))
                && ((bl == b'M' && tr == b'S') || (bl == b'S' && tr == b'M'))
            {
                count += 1;
            }
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_p1() {
        let input = "..X...
.SAMX.
.A..A.
XMAS.S
.X....
......"
            .to_string();
        assert_eq!(part1(input), "4");

        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_string();
        assert_eq!(part1(input), "18");
    }

    #[test]
    fn sample_p2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_string();
        assert_eq!(part2(input), "9");
    }
}
