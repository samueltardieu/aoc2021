use cached::proc_macro::cached;
use itertools::Itertools;

#[aoc(day21, part1)]
fn part1(s: &str) -> u64 {
    let l = s.lines().flat_map(|l| l.split(": ").nth(1)).collect_vec();
    score(l[0].parse().unwrap(), 0, l[1].parse().unwrap(), 0, 1)
}

fn score(p1: u64, s1: u64, p2: u64, s2: u64, c: u64) -> u64 {
    match (p1 + c * 9 - 4) % 10 + 1 {
        p1 if s1 + p1 >= 1000 => c * 3 * s2,
        p1 => score(p2, s2, p1, s1 + p1, c + 1),
    }
}

#[aoc(day21, part2)]
fn part2(s: &str) -> u64 {
    let l = s.lines().flat_map(|l| l.split(": ").nth(1)).collect_vec();
    let (v1, v2) = wins(l[0].parse().unwrap(), 0, l[1].parse().unwrap(), 0);
    v1.max(v2)
}

#[cached]
fn wins(p1: u64, s1: u64, p2: u64, s2: u64) -> (u64, u64) {
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
        .into_iter()
        .fold((0, 0), |(v1, v2), (d, n)| {
            let p1 = (p1 + d - 1) % 10 + 1;
            let (w1, w2) = (s1 + p1 < 21)
                .then(|| wins(p2, s2, p1, s1 + p1))
                .unwrap_or((1, 0));
            (v1 + w2 * n, v2 + w1 * n)
        })
}
