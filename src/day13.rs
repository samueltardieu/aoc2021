use itertools::Itertools;
use pathfinding::prelude::Grid;

fn generator(input: &str) -> (Grid, Vec<(u8, usize)>) {
    let mut lines = input.lines();
    let grid = lines
        .take_while_ref(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    let ins = lines
        .skip(1)
        .map(|line| {
            let (left, right) = line.split_once('=').unwrap();
            (left.bytes().rev().next().unwrap(), right.parse().unwrap())
        })
        .collect();
    (grid, ins)
}

#[aoc(day13, part1)]
fn part1(input: &str) -> usize {
    let (grid, ins) = generator(input);
    fold(grid, ins[0]).vertices_len()
}

#[aoc(day13, part2)]
fn part2(input: &str) -> String {
    let (grid, ins) = generator(input);
    format!("\n{:?}", ins.into_iter().fold(grid, |g, i| fold(g, i)))
}

fn fold(grid: Grid, (axis, n): (u8, usize)) -> Grid {
    let absdiff = |x: usize, y: usize| x.max(y) - x.min(y);
    if axis == b'x' {
        let offset = grid.width.saturating_sub(n * 2 + 1);
        grid.iter()
            .filter_map(|(x, y)| (x != n).then(|| (offset + n - absdiff(n, x), y)))
            .collect()
    } else {
        let offset = grid.height.saturating_sub(n * 2 + 1);
        grid.iter()
            .filter_map(|(x, y)| (y != n).then(|| (x, offset + n - absdiff(n, y))))
            .collect()
    }
}
