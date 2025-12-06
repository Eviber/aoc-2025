pub fn solve(s: &str) -> u64 {
    let mut lines: Vec<&str> = s.lines().collect();
    let op_list: Vec<_> = lines.pop().unwrap().split_whitespace().collect();
    let chars: Vec<_> = lines
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();
    let mut rotated = vec![vec![' '; chars.len()]; chars[0].len() + 1];
    for (y, row) in chars.into_iter().enumerate() {
        for (x, cell) in row.into_iter().enumerate() {
            rotated[x][y] = cell;
        }
    }
    let val_strs: Vec<String> = rotated
        .into_iter()
        .map(|v| v.into_iter().collect::<String>())
        .collect();
    let values: Vec<Vec<u64>> = val_strs
        .split(|s| s.trim().is_empty())
        .map(|t| t.iter().map(|s| s.trim().parse().unwrap()).collect())
        .filter(|v: &Vec<u64>| !v.is_empty())
        .collect();
    let mut total = 0;
    for (op, values) in op_list.into_iter().zip(values.into_iter()) {
        let mut res = 0;
        if op == "*" {
            res += 1;
        }
        for cell in &values {
            if op == "*" {
                res *= cell;
            } else {
                res += cell;
            }
        }
        total += res;
    }
    total
}
