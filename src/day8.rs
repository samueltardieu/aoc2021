use itertools::Itertools;
use std::collections::BTreeSet;

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split_once(" | ")
                .unwrap()
                .1
                .split(' ')
                .filter(|s| !(5..=6).contains(&s.len()))
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    input.lines().map(handle_line).sum()
}

fn handle_line(line: &str) -> usize {
    let (ps, cs) = line.split_once(" | ").unwrap();
    let patterns = ps
        .split(' ')
        .map(|s| s.bytes().collect::<BTreeSet<_>>())
        .collect::<Vec<_>>();
    let find = |len, cond: &dyn Fn(&BTreeSet<u8>) -> bool| {
        patterns.iter().find(|p| p.len() == len && cond(p)).unwrap()
    };
    let one = find(2, &|_| true);
    let seven = find(3, &|_| true);
    let four = find(4, &|_| true);
    let eight = find(7, &|_| true);
    let three = find(5, &|p| p.intersection(one).count() == 2);
    let five = find(5, &|p| p != three && p.intersection(four).count() == 3);
    let two = find(5, &|p| p != three && p != five);
    let nine = find(6, &|p| p.intersection(three).count() == 5);
    let zero = find(6, &|p| p != nine && p.intersection(one).count() == 2);
    let six = find(6, &|p| p != nine && p != zero);
    let patterns = [zero, one, two, three, four, five, six, seven, eight, nine];
    cs.split(' ')
        .map(|s| {
            let c = s.bytes().collect::<BTreeSet<_>>();
            patterns.iter().find_position(|p| p == &&&c).unwrap().0
        })
        .fold(0, |s, d| s * 10 + d)
}
