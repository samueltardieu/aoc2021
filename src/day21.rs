use cached::proc_macro::cached;

fn starting_positions(input: &str) -> (u64, u64) {
    let input = input.trim().as_bytes();
    (
        (input[input.len() / 2 - 1] - b'0') as u64,
        (input[input.len() - 1] - b'0') as u64,
    )
}

#[aoc(day21, part1)]
fn part1(input: &str) -> u64 {
    let (mut p1, mut p2) = starting_positions(input);
    let (mut s1, mut s2, mut d) = (0, 0, 0);
    for i in 0.. {
        up(&mut p1, &mut s1, &mut d);
        if s1 >= 1000 {
            return (6 * i + 3) * s2;
        }
        up(&mut p2, &mut s2, &mut d);
        if s2 >= 1000 {
            return (6 * i + 6) * s1;
        }
    }
    0
}

fn up(p: &mut u64, s: &mut u64, d: &mut u64) {
    for _ in 0..3 {
        *d = *d % 100 + 1;
        *p += *d;
    }
    *p = (*p - 1) % 10 + 1;
    *s += *p;
}

#[aoc(day21, part2)]
fn part2(input: &str) -> u64 {
    let (p1, p2) = starting_positions(input);
    let (v1, v2) = wins(p1, 0, p2, 0);
    v1.max(v2)
}

#[cached]
fn wins(p1: u64, s1: u64, p2: u64, s2: u64) -> (u64, u64) {
    if s2 < 21 {
        [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
            .into_iter()
            .fold((0, 0), |(v1, v2), (d, n)| {
                let p1 = (p1 + d - 1) % 10 + 1;
                let (w1, w2) = wins(p2, s2, p1, s1 + p1);
                (v1 + w2 * n, v2 + w1 * n)
            })
    } else {
        (0, 1)
    }
}
