#[aoc(day10, part1)]
fn part1(input: &str) -> u64 {
    input.lines().map(syntax_error_score).sum()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    let mut scores = input.lines().flat_map(incomplete_score).collect::<Vec<_>>();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn syntax_error_score(line: &str) -> u64 {
    let mut stack = Vec::new();
    for c in line.bytes() {
        match c {
            b')' | b']' | b'}' | b'>' => match stack.pop() {
                Some(o) if o == c - 1 || o == c - 2 => (),
                _ => return [3, 25137, 57, 1197][c as usize / 30 - 1],
            },
            c => stack.push(c),
        }
    }
    return 0;
}

fn incomplete_score(line: &str) -> Option<usize> {
    let mut stack = Vec::new();
    for c in line.bytes() {
        match c {
            b')' | b']' | b'}' | b'>' => {
                if !(1..=2).contains(&c.wrapping_sub(stack.pop()?)) {
                    return None;
                }
            }
            c => stack.push(c),
        }
    }
    Some(
        stack
            .into_iter()
            .rev()
            .fold(0, |s, c| s * 5 + [1, 4, 2, 3][c as usize / 30 - 1]),
    )
}
