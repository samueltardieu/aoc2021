#[aoc(day7, part1, separator = ',')]
fn part1(mut pos: Vec<i32>) -> i32 {
    let target = median(&mut pos);
    pos.into_iter().map(|p| (p - target).abs()).sum()
}

#[aoc(day7, part2, separator = ',')]
fn part2(pos: &[i32]) -> i32 {
    let mean = pos.iter().sum::<i32>() / pos.len() as i32;
    cost(pos, mean).min(cost(pos, mean + 1))
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
