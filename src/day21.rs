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

struct State {
    p1: u64,
    s1: u64,
    p2: u64,
    s2: u64,
    count: usize,
}

#[aoc(day21, part2)]
fn part2(input: &str) -> usize {
    const DICE: [(u64, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
    let (p1, p2) = starting_positions(input);
    let mut states = vec![State {
        p1,
        s1: 0,
        p2,
        s2: 0,
        count: 1,
    }];
    let (mut v1, mut v2) = (0, 0);
    while !states.is_empty() {
        let mut ns = vec![];
        for state in states.drain(..) {
            for (d1, occ1) in DICE {
                let p1 = (state.p1 + d1 - 1) % 10 + 1;
                let s1 = state.s1 + p1;
                if s1 >= 21 {
                    v1 += state.count * occ1;
                } else {
                    for (d2, occ2) in DICE {
                        let p2 = (state.p2 + d2 - 1) % 10 + 1;
                        let s2 = state.s2 + p2;
                        if s2 >= 21 {
                            v2 += state.count * occ1 * occ2;
                        } else {
                            ns.push(State {
                                p1,
                                s1,
                                p2,
                                s2,
                                count: state.count * occ1 * occ2,
                            })
                        }
                    }
                }
            }
        }
        states = ns;
    }
    v1.max(v2)
}
