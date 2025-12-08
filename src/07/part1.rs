pub fn solve(s: &str) -> usize {
    let mut lines = s.lines();
    let mut beams: Vec<bool> = lines.next().unwrap().chars().map(|c| c == 'S').collect();
    let mut count = 0;
    for line in lines {
        let mut next_beam = vec![false; beams.len()];
        for (i, cell) in beams
            .into_iter()
            .zip(line.chars())
            .enumerate()
            .filter(|&(_, (has_beam, _))| has_beam)
            .map(|(i, (_, cell))| (i, cell))
        {
            if cell == '.' {
                next_beam[i] = true;
                continue;
            }
            assert_eq!(cell, '^');
            next_beam[i - 1] = true;
            next_beam[i + 1] = true;
            count += 1;
        }
        beams = next_beam;
    }
    count
}
