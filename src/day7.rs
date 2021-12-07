fn parse(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

#[aoc(day7, part1)]
fn part1(input: &str) -> i32 {
    let mut pos = parse(input);
    let target = median(&mut pos);
    pos.into_iter().map(|p| (p - target).abs()).sum()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> i32 {
    let pos = parse(input);
    let mean = pos.iter().sum::<i32>() / pos.len() as i32;
    cost(&pos, mean).min(cost(&pos, mean + 1))
}

fn median(v: &mut Vec<i32>) -> i32 {
    v.sort_unstable();
    if v.len() % 2 == 1 {
        v[v.len() / 2]
    } else {
        (v[v.len() / 2 - 1] + v[v.len() / 2]) / 2
    }
}

fn cost(pos: &[i32], target: i32) -> i32 {
    pos.iter()
        .map(|&p| {
            let n = (p - target).abs();
            n * (n + 1) / 2
        })
        .sum()
}
