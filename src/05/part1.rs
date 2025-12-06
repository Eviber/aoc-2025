fn parse(s: &str) -> u64 {
    s.parse().unwrap()
}

pub fn solve(s: &str) -> usize {
    let (ranges, ids) = s.split_once("\n\n").unwrap();
    let ranges: Vec<_> = ranges
        .split_whitespace()
        .map(|s| s.split_once('-').unwrap())
        .map(|(a, b)| (parse(a), parse(b)))
        .collect();
    ids.split_whitespace()
        .map(parse)
        .filter(|&id| {
            for &(a, b) in &ranges {
                if id >= a && id <= b {
                    return true;
                }
            }
            false
        })
        .count()
}
