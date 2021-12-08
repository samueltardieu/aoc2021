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
                .map(|s| s.len())
                .filter(|&l| l == 2 || l == 3 || l == 4 || l == 7)
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
    let one = patterns.iter().find(|p| p.len() == 2).unwrap();
    let seven = patterns.iter().find(|p| p.len() == 3).unwrap();
    let four = patterns.iter().find(|p| p.len() == 4).unwrap();
    let eight = patterns.iter().find(|p| p.len() == 7).unwrap();
    let three = patterns
        .iter()
        .find(|p| p.len() == 5 && p.intersection(one).count() == 2)
        .unwrap();
    let five = patterns
        .iter()
        .find(|p| p.len() == 5 && p != &three && p.intersection(four).count() == 3)
        .unwrap();
    let two = patterns
        .iter()
        .find(|p| p.len() == 5 && p != &three && p != &five)
        .unwrap();
    let nine = patterns
        .iter()
        .find(|p| p.len() == 6 && p.intersection(three).count() == 5)
        .unwrap();
    let zero = patterns
        .iter()
        .find(|p| p.len() == 6 && p != &nine && p.intersection(one).count() == 2)
        .unwrap();
    let six = patterns
        .iter()
        .find(|p| p.len() == 6 && p != &nine && p != &zero)
        .unwrap();
    let patterns = [zero, one, two, three, four, five, six, seven, eight, nine];
    cs.split(' ')
        .map(|s| {
            let c = s.bytes().collect::<BTreeSet<_>>();
            patterns.iter().find_position(|p| p == &&&c).unwrap().0
        })
        .fold(0, |s, d| s * 10 + d)
}
