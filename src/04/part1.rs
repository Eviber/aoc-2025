fn new_row(row: &str) -> Vec<char> {
    row.chars().collect()
}

#[allow(clippy::needless_range_loop)]
fn neighbours(map: &[Vec<char>], row: usize, col: usize) -> usize {
    let mut neighbours = 0;
    for nrow in row.saturating_sub(1)..map.len().min(row + 2) {
        for ncol in col.saturating_sub(1)..map[0].len().min(col + 2) {
            if (nrow != row || ncol != col) && map[nrow][ncol] == '@' {
                neighbours += 1;
            }
        }
    }
    neighbours
}

pub fn solve(s: &str) -> u64 {
    let map: Vec<_> = s.split_whitespace().map(new_row).collect();
    let mut count = 0;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == '@' && neighbours(&map, row, col) < 4 {
                count += 1;
            }
        }
    }
    count
}
