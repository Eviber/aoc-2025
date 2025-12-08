pub fn solve(s: &str) -> usize {
    let mut lines = s.lines();
    let mut beams: Vec<usize> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| (c == 'S') as usize)
        .collect();
    for line in lines {
        let mut next_beam = vec![0; beams.len()];
        for (i, beam_count, cell) in beams
            .into_iter()
            .zip(line.chars())
            .enumerate()
            .filter(|&(_, (beam_count, _))| beam_count != 0)
            .map(|(i, (beam_count, cell))| (i, beam_count, cell))
        {
            if cell == '.' {
                next_beam[i] += beam_count;
                continue;
            }
            assert_eq!(cell, '^');
            next_beam[i - 1] += beam_count;
            next_beam[i + 1] += beam_count;
        }
        beams = next_beam;
    }
    beams.into_iter().sum()
}
