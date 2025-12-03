fn is_repeat_with_mod(n: u64, n_mod: u64) -> bool {
    let val = n % n_mod;
    let mut n_div = n_mod;
    while n_div <= n {
        if (n / n_div) % n_mod != val {
            return false;
        }
        n_div *= n_mod;
    }
    true
}

fn is_repeated(&n: &u64) -> bool {
    let n_log10 = n.ilog10();
    let max_digit_check = n_log10.div_ceil(2);
    let mut n_mod = 1;
    for n_digit in 1..=max_digit_check {
        n_mod *= 10;
        if !(n_log10 + 1).is_multiple_of(n_digit) {
            continue;
        }
        if is_repeat_with_mod(n, n_mod) {
            return true;
        }
    }
    false
}

pub fn solve(s: &str) -> u64 {
    s.trim()
        .split(',')
        .map(|s| s.split_once('-').unwrap())
        .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
        .flat_map(|(start, end)| {
            (start..=end).filter(is_repeated)
        })
        .sum()
}
