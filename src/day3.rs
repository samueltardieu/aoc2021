fn generator(input: &str) -> (usize, Vec<u32>) {
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    (
        width,
        lines
            .into_iter()
            .map(|l| u32::from_str_radix(l, 2).unwrap())
            .collect(),
    )
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let (width, numbers) = generator(input);
    let mcb: u32 = (0..width)
        .map(|x| 1u32 << x)
        .filter(|x| numbers.iter().filter(|n| *n & *x != 0).count() * 2 > numbers.len())
        .sum();
    mcb * (((1 << width) - 1) ^ mcb)
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let (width, mut numbers) = generator(input);
    select(width, &mut numbers, false) * select(width, &mut numbers, true)
}

fn select(width: usize, numbers: &mut [u32], invert: bool) -> u32 {
    if numbers.len() == 1 {
        return numbers[0];
    }
    let index = itertools::partition(&mut *numbers, |n| n & (1 << (width - 1)) != 0);
    if invert ^ (index * 2 >= numbers.len()) {
        select(width - 1, &mut numbers[..index], invert)
    } else {
        select(width - 1, &mut numbers[index..], invert)
    }
}
