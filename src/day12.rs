use std::collections::{BTreeSet, HashMap};

fn generator(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut successors = HashMap::new();
    input.lines().for_each(|l| {
        let (start, end) = l.split_once('-').unwrap();
        successors.entry(start).or_insert_with(Vec::new).push(end);
        successors.entry(end).or_insert_with(Vec::new).push(start);
    });
    successors
}

#[aoc(day12, part1)]
fn part1(input: &str) -> usize {
    let successors = generator(input);
    paths("start", &successors, &mut BTreeSet::new(), false)
}

#[aoc(day12, part2)]
fn part2(input: &str) -> usize {
    let successors = generator(input);
    paths("start", &successors, &mut BTreeSet::new(), true)
}

fn paths<'a>(
    node: &'a str,
    successors: &HashMap<&'a str, Vec<&'a str>>,
    seen: &mut BTreeSet<&'a str>,
    may_double: bool,
) -> usize {
    let s = seen.contains(&node);
    match node {
        "end" => 1,
        _ if s && !may_double => 0,
        _ => {
            let inserted = !s && node.as_bytes()[0] >= b'a' && seen.insert(node);
            let count = successors[&node]
                .iter()
                .filter(|&&n| n != "start")
                .map(|&n| paths(n, successors, seen, may_double && !s))
                .sum();
            if inserted {
                seen.remove(&node);
            }
            count
        }
    }
}
