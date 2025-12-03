fn step_over_zero(mut n: i32) -> i32 {
    if n <= 0 {
        n -= 1;
    }
    n.div_euclid(100).abs()
}

pub fn solve(s: &str) -> i32 {
    s.split_whitespace()
        .map(|s| {
            let (dir, n) = s.split_at(1);
            match dir {
                "R" => n.parse::<i32>().unwrap(),
                "L" => -n.parse::<i32>().unwrap(),
                _ => unreachable!(),
            }
        })
        .fold((0, 50), |(mut count, start), n: i32| {
            let delta = start + n;
            count += step_over_zero(delta);
            if start == 0 && delta < 0 {
                count -= 1;
            }
            let end = delta.rem_euclid(100);
            (count, end)
        })
        .0
}
