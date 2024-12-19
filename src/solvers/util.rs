pub fn adjacent_in_bounds(
    i: usize,
    j: usize,
    matrix_len: usize,
) -> impl Iterator<Item = (usize, usize)> {
    const ADJACENTS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)]; // up down left right
    return ADJACENTS.iter().filter_map(move |(dy, dx)| {
        let pair = (i as i32 + *dy, j as i32 + *dx);
        let legal_range = 0..(matrix_len as i32);
        if legal_range.contains(&pair.0) && legal_range.contains(&pair.1) {
            return Some((pair.0 as usize, pair.1 as usize));
        } else {
            return None;
        }
    });
}

pub fn get_2d<T>(matrix: &Vec<Vec<T>>, (y, x): (usize, usize)) -> Option<&T> {
    matrix.get(y).and_then(|row| row.get(x))
}

pub fn float_basically_integer(n: f64, threshold_exp: i32) -> Option<u64> {
    // essentially python math.isclose() which checks if this is basically an integer
    let rounded = n.round();
    if (rounded - n).abs() < 10.0f64.powi(threshold_exp) {
        Some(rounded as u64)
    } else {
        None
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn apply_unchecked(&self, (y, x): (usize, usize)) -> (usize, usize) {
        // no bounds checking required as the area is padded
        match self {
            Direction::Up => (y - 1, x),
            Direction::Down => (y + 1, x),
            Direction::Left => (y, x - 1),
            Direction::Right => (y, x + 1),
        }
    }
}
