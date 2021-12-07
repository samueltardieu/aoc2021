fn days(input: &[usize], days: usize) -> usize {
    let mut fishes = input.iter().fold([0usize; 9], |mut fishes, &f| {
        fishes[f] += 1;
        fishes
    });
    (0..days).for_each(|d| fishes[(d + 7) % 9] += fishes[d % 9]);
    fishes.into_iter().sum()
}

#[aoc(day6, part1, separator = ',')]
fn part1(input: &[usize]) -> usize {
    days(input, 80)
}

#[aoc(day6, part2, separator = ',')]
fn part2(input: &[usize]) -> usize {
    days(input, 256)
}
