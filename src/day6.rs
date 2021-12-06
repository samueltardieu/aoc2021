fn days(input: &str, days: usize) -> usize {
    let mut fishes = input.trim().split(',').fold([0usize; 9], |mut fishes, s| {
        fishes[s.parse::<usize>().unwrap()] += 1;
        fishes
    });
    (0..days).for_each(|d| fishes[(d + 7) % 9] += fishes[d % 9]);
    fishes.into_iter().sum()
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    days(input, 80)
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    days(input, 256)
}
