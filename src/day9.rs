use itertools::Itertools;
use pathfinding::prelude::Matrix;
use std::collections::BTreeMap;

fn generator(input: &str) -> Matrix<i32> {
    let lines = input
        .lines()
        .map(|c| c.bytes().map(|c| (c - b'0') as i32).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut m = Matrix::new_empty(lines[0].len());
    lines.into_iter().for_each(|l| m.extend(&l).unwrap());
    m
}

#[aoc(day9, part1)]
fn part1(input: &str) -> i32 {
    let m = generator(input);
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
    let m = generator(input);
    let mut basins = BTreeMap::<(usize, usize), usize>::new();
    for k in (0..m.rows).cartesian_product(0..m.columns) {
        let (mut k, mut v) = (k, m[&k]);
        while let Some(n) = (v != 9)
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
