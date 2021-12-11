use pathfinding::prelude::Matrix;
use std::collections::BTreeSet;

fn generator(input: &str) -> Matrix<u32> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()))
        .collect()
}

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    (0..100)
        .scan(generator(input), |map, _| Some(step(map)))
        .sum()
}

#[aoc(day11, part2)]
fn part2(input: &str) -> usize {
    let mut map = generator(input);
    (1..)
        .flat_map(|n| (step(&mut map) == map.len()).then(|| n))
        .next()
        .unwrap()
}

fn step(map: &mut Matrix<u32>) -> usize {
    map.iter_mut().for_each(|v| *v = (*v + 1) % 10);
    let mut to_see = map.indices().filter(|&i| map[i] == 0).collect::<Vec<_>>();
    let mut seen = BTreeSet::from_iter(to_see.clone());
    while let Some(idx) = to_see.pop() {
        for n in map
            .neighbours(idx, true)
            .filter(|n| !seen.contains(n))
            .collect::<Vec<_>>()
        {
            map[n] = (map[n] + 1) % 10;
            if map[n] == 0 {
                seen.insert(n);
                to_see.push(n);
            }
        }
    }
    seen.len()
}
