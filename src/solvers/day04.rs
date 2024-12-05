use arrayvec::ArrayVec;

fn try_apply_direction(
    y: usize,
    x: usize,
    (dy, dx): (i16, i16),
    arr: &ArrType,
    mul: i16,
) -> Option<&char> {
    let ny = (y as i16 + dy * mul) as usize;
    let nx = (x as i16 + dx * mul) as usize;
    arr.get(ny).and_then(|it| it.get(nx))
}

type ArrType = Vec<Vec<char>>;

pub fn part1(input: String) -> String {
    let mat = input
        .lines()
        // utf32, could be u8 ascii or the nightly-only char_ascii (https://github.com/rust-lang/rust/issues/110998)
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

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
                'X' => ['X', 'M', 'A', 'S'],
                'S' => ['S', 'A', 'M', 'X'],
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
    let mat = input
        .lines()
        // utf32, todo: utf8/ascii?
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // ensure input is square
    assert!(mat.iter().all(|line| line.len() == mat.len()));

    let mut count = 0;
    for y in 0..mat.len() {
        for x in 0..mat.len() {
            // we only want to check the middle of the X-MAS
            // as we're doing it by checking the diagonals
            if mat[y][x] != 'A' {
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
                .collect::<ArrayVec<char, 4>>();
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
            if ((tl == 'M' && br == 'S') || (tl == 'S' && br == 'M'))
                && ((bl == 'M' && tr == 'S') || (bl == 'S' && tr == 'M'))
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
