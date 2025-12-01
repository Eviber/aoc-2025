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
        .fold((0, 50), |(mut count, mut acc), n: i32| {
            acc = (acc + n).rem_euclid(100);
            if acc == 0 {
                count += 1;
            }
            (count, acc)
        }).0
}
