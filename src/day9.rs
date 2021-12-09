use itertools::Itertools;
use pathfinding::prelude::Matrix;
use std::collections::BTreeMap;

#[aoc(day9, part1)]
fn part1(input: &str) -> i32 {
    let m = Matrix::from_rows(input.lines().map(|c| c.bytes().map(|c| (c - b'0') as i32))).unwrap();
    (0..m.rows)
        .cartesian_product(0..m.columns)
        .filter_map(|(r, c)| {
            let v = m[&(r, c)];
            m.neighbours(&(r, c), false)
                .all(|n| m[&n] > v)
                .then(|| v + 1)
        })
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    let m = Matrix::from_rows(input.lines().map(|c| c.bytes())).unwrap();
    let mut basins = BTreeMap::<(usize, usize), usize>::new();
    for k in (0..m.rows).cartesian_product(0..m.columns) {
        let (mut k, mut v) = (k, m[&k]);
        while let Some(n) = (v != b'9')
            .then(|| m.neighbours(&k, false).find(|n| m[n] < v))
            .flatten()
        {
            k = n;
            v = m[&k];
        }
        *basins.entry(k).or_insert(0) += 1;
    }
    basins.values().sorted().rev().take(3).product()
}
