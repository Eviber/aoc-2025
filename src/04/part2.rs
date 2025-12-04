fn new_row(row: &str) -> Vec<bool> {
    row.chars().map(|c| c == '@').collect()
}

fn neighbours(
    width: usize,
    height: usize,
    row: usize,
    col: usize,
) -> impl Iterator<Item = (usize, usize)> {
    (row.saturating_sub(1)..((row + 2).min(height)))
        .flat_map(move |row| {
            (col.saturating_sub(1)..((col + 2).min(width))).map(move |col| (row, col))
        })
        .filter(move |&(nrow, ncol)| nrow != row || ncol != col)
}

fn all_neighbours_count(map: &[Vec<bool>]) -> Vec<Vec<usize>> {
    let mut n_count = vec![vec![0; map[0].len()]; map.len()];
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if !map[row][col] {
                continue;
            }
            for (nrow, ncol) in neighbours(map.len(), map[0].len(), row, col) {
                n_count[nrow][ncol] += 1;
            }
        }
    }
    n_count
}

pub fn solve(s: &str) -> u64 {
    let mut map: Vec<_> = s.split_whitespace().map(new_row).collect();
    let mut n_count = all_neighbours_count(&map);
    let mut count = 0;
    let mut queue: Vec<(usize, usize)> = (0..map.len())
        .flat_map(|row| (0..map[0].len()).map(move |col| (row, col)))
        .filter(|&(row, col)| map[row][col])
        .collect();
    while let Some((row, col)) = queue.pop() {
        if !map[row][col] || n_count[row][col] >= 4 {
            continue;
        }
        count += 1;
        map[row][col] = false;
        for (nrow, ncol) in neighbours(map.len(), map[0].len(), row, col) {
            n_count[nrow][ncol] -= 1;
            if n_count[nrow][ncol] < 4 {
                queue.push((nrow, ncol));
            }
        }
    }
    count
}
