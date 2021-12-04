use pathfinding::prelude::Matrix;

type Bingo = (Vec<u32>, Vec<Matrix<u32>>);

fn generator(input: &str) -> Bingo {
    let mut lines = input.lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let mut grids = Vec::new();
    while let Some(_empty) = lines.next() {
        let mut m = Matrix::new(5, 5, 0);
        for r in 0..5 {
            for (c, x) in lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .enumerate()
            {
                m[&(r, c)] = x;
            }
        }
        grids.push(m);
    }
    (numbers, grids)
}

fn is_complete(m: &Matrix<bool>) -> bool {
    for r in 0..5 {
        if (0..5).map(|c| m[&(r, c)]).all(|b| b) || (0..5).map(|c| m[&(c, r)]).all(|b| b) {
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
    let (numbers, mut grids) = generator(input);
    let mut found = Vec::new();
    found.resize(grids.len(), Matrix::new(5, 5, false));
    let mut complete = Vec::new();
    complete.resize(grids.len(), false);
    for n in numbers {
        for (i, (g, f)) in grids.iter_mut().zip(found.iter_mut()).enumerate() {
            let mut score = 0;
            for (gg, ff) in g.values_mut().zip(f.values_mut()) {
                if *gg == n {
                    *ff = true;
                }
                if !*ff {
                    score += *gg;
                }
            }
            if is_complete(f) {
                complete[i] = true;
                if is_part1 || complete.iter().all(|b| *b) {
                    return score * n;
                }
            }
        }
    }
    0
}
