use std::collections::BTreeMap;

type Template = BTreeMap<(u8, u8), usize>;
type Rules = BTreeMap<(u8, u8), u8>;

fn generator(input: &str) -> (Template, Rules) {
    let mut lines = input.lines();
    let line = lines.next().unwrap().as_bytes();
    let template = (0..line.len() - 1)
        .map(|i| ((line[i], line[i + 1]), 1usize))
        .collect::<BTreeMap<_, _>>();
    let rules = lines
        .skip(1)
        .map(|l| {
            let l = l.as_bytes();
            ((l[0], l[1]), l[l.len() - 1])
        })
        .collect();
    (template, rules)
}

#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
    part(input, 10)
}

#[aoc(day14, part2)]
fn part2(input: &str) -> usize {
    part(input, 40)
}

fn part(input: &str, steps: usize) -> usize {
    let (template, rules) = generator(input);
    max_diff(&(0..steps).fold(template, |t, _| step(&t, &rules)))
}

fn max_diff(template: &Template) -> usize {
    let mut elems = BTreeMap::new();
    for (&(a, b), &n) in template {
        *elems.entry(a).or_insert(0) += n;
        *elems.entry(b).or_insert(0) += n;
    }
    (elems.values().max().unwrap() + 1) / 2 - (elems.values().min().unwrap() + 1) / 2 // start and end are odd
}

fn step(template: &Template, rules: &Rules) -> Template {
    let mut res = BTreeMap::new();
    for (&t @ (a, b), n) in template {
        if let Some(&c) = rules.get(&t) {
            *res.entry((a, c)).or_insert(0) += n;
            *res.entry((c, b)).or_insert(0) += n;
        } else {
            *res.entry(t).or_insert(0) += n;
        }
    }
    res
}
