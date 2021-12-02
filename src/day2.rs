fn commands(input: &str) -> Vec<(&str, i32)> {
    input
        .lines()
        .map(|s| {
            let s = s.split_once(' ').unwrap();
            (s.0, s.1.parse().unwrap())
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &str) -> i32 {
    let (mut h, mut v) = (0, 0);
    for (c, x) in commands(input) {
        match c {
            "forward" => h += x,
            "down" => v += x,
            _ => v -= x,
        }
    }
    h * v
}

#[aoc(day2, part2)]
fn part2(input: &str) -> i32 {
    let (mut h, mut v, mut a) = (0, 0, 0);
    for (c, x) in commands(input) {
        match c {
            "forward" => {
                h += x;
                v += a * x
            }
            "down" => a += x,
            _ => a -= x,
        }
    }
    h * v
}
