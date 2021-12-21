use cached::proc_macro::cached;

fn starting_positions(input: &str) -> (u64, u64) {
    let input = input.trim().as_bytes();
    (
        (input[input.len() / 2 - 1] - b'0') as u64,
        (input.last().unwrap() - b'0') as u64,
    )
}

#[aoc(day21, part1)]
fn part1(input: &str) -> u64 {
    let (p1, p2) = starting_positions(input);
    score(p1, 0, p2, 0, 1)
}

fn score(p1: u64, s1: u64, p2: u64, s2: u64, c: u64) -> u64 {
    match (p1 + c * 9 - 4) % 10 + 1 {
        p1 if s1 + p1 >= 1000 => c * 3 * s2,
        p1 => score(p2, s2, p1, s1 + p1, c + 1),
    }
}

#[aoc(day21, part2)]
fn part2(input: &str) -> u64 {
    let (p1, p2) = starting_positions(input);
    let (v1, v2) = wins(p1, 0, p2, 0);
    v1.max(v2)
}

#[cached]
fn wins(p1: u64, s1: u64, p2: u64, s2: u64) -> (u64, u64) {
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
        .into_iter()
        .fold((0, 0), |(v1, v2), (d, n)| {
            let p1 = (p1 + d - 1) % 10 + 1;
            if s1 + p1 >= 21 {
                (v1 + n, v2)
            } else {
                let (w1, w2) = wins(p2, s2, p1, s1 + p1);
                (v1 + w2 * n, v2 + w1 * n)
            }
        })
}
