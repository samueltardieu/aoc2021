fn days(input: &str, days: usize) -> usize {
    let mut fishes = [0usize; 9];
    for f in input.trim().split(',').map(|s| s.parse::<usize>().unwrap()) {
        fishes[f] += 1;
    }
    for d in 0..days {
        fishes[(d + 7) % 9] += fishes[d % 9];
    }
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
