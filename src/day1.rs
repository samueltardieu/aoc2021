#[aoc(day1, part1)]
fn part1(input: &[u32]) -> usize {
    input.windows(2).filter(|v| v[1] > v[0]).count()
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> usize {
    input.windows(4).filter(|v| v[3] > v[0]).count()
}
