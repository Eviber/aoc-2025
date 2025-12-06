pub fn solve(s: &str) -> u64 {
    let mut lines: Vec<&str> = s.lines().collect();
    let op_list: Vec<_> = lines.pop().unwrap().split_whitespace().collect();
    let values: Vec<_> = lines
        .into_iter()
        .map(|s| {
            s.split_whitespace()
                .map(|w| w.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let mut total = 0;
    for x in 0..op_list.len() {
        let mut res = 0;
        let op = op_list[x];
        if op == "*" {
            res += 1;
        }
        for row in &values {
            let cell = row[x];
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
