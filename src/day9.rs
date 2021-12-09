use itertools::{iproduct, Itertools};
use pathfinding::prelude::{bfs_reach, Matrix};

fn lows(m: &Matrix<u8>) -> impl Iterator<Item = (usize, usize)> + '_ {
    iproduct!(0..m.rows, 0..m.columns).filter(|k| m.neighbours(k, false).all(|n| m[&n] > m[k]))
}

#[aoc(day9, part1)]
fn part1(input: &str) -> u32 {
    let m = Matrix::from_rows(input.lines().map(|c| c.bytes())).unwrap();
    lows(&m).map(|k| (m[&k] - b'0') as u32 + 1).sum()
}

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    let m = Matrix::from_rows(input.lines().map(|c| c.bytes())).unwrap();
    lows(&m)
        .map(|n| {
            bfs_reach(n, |n| {
                m.neighbours(n, false)
                    .filter(|k| m[k] != b'9' && m[k] > m[n])
                    .collect_vec()
            })
            .count()
        })
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(3)
        .product()
}
