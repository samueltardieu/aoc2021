fn median(v: &mut Vec<i32>) -> i32 {
    v.sort_unstable();
    if v.len() % 2 == 1 {
        v[v.len() / 2]
    } else {
        (v[v.len() / 2 - 1] + v[v.len() / 2]) / 2
    }
}

fn parse(input: &str) -> (Vec<i32>, i32) {
    let mut pos = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let median = median(&mut pos);
    (pos, median)
}

#[aoc(day7, part1)]
fn part1(input: &str) -> i32 {
    let (pos, target) = parse(input);
    pos.into_iter().map(|p| (p - target).abs()).sum()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> i32 {
    let (pos, mut target) = parse(input);
    let mut fuel = cost(&pos, target);
    let dir = (fuel - cost(&pos, target + 1)).signum();
    loop {
        let new_fuel = cost(&pos, target + dir);
        if new_fuel >= fuel {
            return fuel as i32;
        }
        target += dir;
        fuel = new_fuel;
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
