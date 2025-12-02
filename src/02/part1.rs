fn is_double(n: &u64) -> bool {
    let digit_count = n.ilog10() + 1;
    let mod_split = 10u64.pow(digit_count / 2);
    n / mod_split == n % mod_split
}

pub fn solve(s: &str) -> u64 {
    s.trim()
        .split(',')
        .map(|s| s.split_once('-').unwrap())
        .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
        .flat_map(|(start, end)| {
            (start..=end).filter(is_double)
        })
        .sum()
}
