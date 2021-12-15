use pathfinding::prelude::{astar, Matrix};

#[aoc(day15, part1)]
fn part1(input: &str) -> u32 {
    part(input, 1)
}

#[aoc(day15, part2)]
fn part2(input: &str) -> u32 {
    part(input, 5)
}

fn part(input: &str, repeat: usize) -> u32 {
    let orig: Matrix<u32> = input
        .lines()
        .map(|l| l.bytes().map(|b| (b - b'0') as u32))
        .collect();
    let (nr, nc) = (orig.rows, orig.columns);
    let mut map = Matrix::new(orig.rows * repeat, orig.columns * repeat, 0);
    for (r, c) in map.indices() {
        map[(r, c)] = (orig[(r % nr, c % nc)] + (r / nr + c / nc) as u32 - 1) % 9 + 1;
    }
    astar(
        &(0, 0),
        |&n| map.neighbours(n, false).map(|n| (n, map[n])),
        |&(x, y)| (map.rows + map.columns - x - y - 2) as u32,
        |&n| n == (map.rows - 1, map.columns - 1),
    )
    .unwrap()
    .1
}
