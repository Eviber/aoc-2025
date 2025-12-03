fn first_largest_digit(v: &[u64]) -> (usize, u64) {
    v.iter()
        .copied()
        .enumerate()
        .max_by(|(a_i, a_n), (b_i, b_n)| {
            if a_n != b_n {
                a_n.cmp(b_n)
            } else {
                b_i.cmp(a_i) // ensure the first occurence is selected
            }
        })
        .expect("v should not be empty")
}

fn largest_n_digit_comb<const N: usize>(v: Vec<u64>) -> u64 {
    let mut res = 0;
    let mut current_idx = 0;
    for end_range in (0..N).rev() {
        res *= 10;
        let (idx, digit) = first_largest_digit(&v[(current_idx)..(v.len() - end_range)]);
        current_idx += idx + 1;
        res += digit;
    }
    res
}

pub fn solve(s: &str) -> u64 {
    s.split_whitespace()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<_>>()
        })
        .map(largest_n_digit_comb::<12>)
        .sum()
}
