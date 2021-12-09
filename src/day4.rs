use pathfinding::prelude::Matrix;

fn generator(input: &str) -> (Vec<u32>, Vec<Matrix<u32>>) {
    let mut lines = input.lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let mut grids = Vec::new();
    while let Some(_empty) = lines.next() {
        grids.push(
            (0..5)
                .map(|_| {
                    lines
                        .next()
                        .unwrap()
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                })
                .collect(),
        );
    }
    (numbers, grids)
}

fn is_complete(m: &Matrix<bool>) -> bool {
    for r in 0..5 {
        if (0..5).map(|c| m[(r, c)]).all(|b| b) || (0..5).map(|c| m[(c, r)]).all(|b| b) {
            return true;
        }
    }
    false
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    solve(input, true)
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u32 {
    solve(input, false)
}

fn solve(input: &str, is_part1: bool) -> u32 {
    let (numbers, grids) = generator(input);
    let mut found = vec![Matrix::new(5, 5, false); grids.len()];
    let mut complete = vec![false; grids.len()];
    for n in numbers {
        for (i, (g, f)) in grids.iter().zip(found.iter_mut()).enumerate() {
            let mut score = 0;
            for (&gg, ff) in g.values().zip(f.values_mut()) {
                *ff |= gg == n;
                score += gg * !*ff as u32;
            }
            if is_complete(f) {
                complete[i] = true;
                if is_part1 || complete.iter().all(|&b| b) {
                    return score * n;
                }
            }
        }
    }
    0
}
