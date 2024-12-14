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
