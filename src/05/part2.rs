fn parse(s: &str) -> (u64, u64) {
    let (a, b) = s.split_once('-').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

pub fn solve(s: &str) -> u64 {
    let (ranges, _) = s.split_once("\n\n").unwrap();
    let mut ranges: Vec<_> = ranges.split_whitespace().map(parse).collect();
    ranges.sort();
    let mut count = 0;
    while let Some(b) = ranges.pop() {
        let Some(a) = ranges.pop() else {
            // b is last element, add its count
            count += b.1 - b.0 + 1;
            break;
        };
        if b.0 > a.1 + 1 {
            // a and b don't overlap, add b's count and keep a
            count += b.1 - b.0 + 1;
            ranges.push(a);
            continue;
        }
        // a and b overlap, keep the merged form
        ranges.push((a.0, b.1.max(a.1)));
    }
    count
}
