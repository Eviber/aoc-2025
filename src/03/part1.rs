fn largest_two_digit_comb(v: Vec<u64>) -> u64 {
    let &(i, digit) = &v[..(v.len() - 1)]
        .iter()
        .enumerate()
        .max_by(|(a_i, a_n), (b_i, b_n)| {
            if a_n != b_n {
                a_n.cmp(b_n)
            } else {
                b_i.cmp(a_i) // ensure the first occurence is selected
            }
        })
        .unwrap();
    digit * 10 + v.into_iter().skip(i + 1).max().unwrap()
}

pub fn solve(s: &str) -> u64 {
    s.split_whitespace()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<_>>()
        })
        .map(largest_two_digit_comb)
        .sum()
}
