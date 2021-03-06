use std::collections::BTreeMap;

type Template = BTreeMap<(u8, u8), usize>;
type Rules = BTreeMap<(u8, u8), u8>;

fn generator(input: &[&[u8]]) -> (Template, Rules) {
    let mut template = Template::new();
    input[0]
        .windows(2)
        .for_each(|w| *template.entry((w[0], w[1])).or_insert(0) += 1);
    let rules = input[2..]
        .iter()
        .map(|l| ((l[0], l[1]), *l.last().unwrap()))
        .collect();
    (template, rules)
}

#[aoc(day14, part1)]
fn part1(input: &[&[u8]]) -> usize {
    let (template, rules) = generator(input);
    max_diff(&(0..10).fold(template, |t, _| step(&t, &rules)))
}

#[aoc(day14, part2)]
fn part2(input: &[&[u8]]) -> usize {
    let (template, rules) = generator(input);
    max_diff(&(0..40).fold(template, |t, _| step(&t, &rules)))
}

fn max_diff(template: &Template) -> usize {
    let mut elems = BTreeMap::new();
    for (&(a, b), &n) in template {
        *elems.entry(a).or_insert(0) += n;
        *elems.entry(b).or_insert(0) += n;
    }
    (elems.values().max().unwrap() + 1) / 2 - (elems.values().min().unwrap() + 1) / 2
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
