use std::{collections::BTreeSet, ops::Sub};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point(i32, i32, i32);

impl Point {
    fn rotate(&self, i: u8) -> Self {
        let p = match i / 8 {
            0 => *self,
            1 => Point(self.1, self.2, self.0),
            _ => Point(self.2, self.0, self.1),
        };
        let p = match i & 4 {
            0 => p,
            _ => Point(-p.0, -p.1, p.2),
        };
        match i % 4 {
            0 => p,
            1 => Point(p.0, -p.2, p.1),
            2 => Point(p.0, -p.1, -p.2),
            _ => Point(p.0, p.2, -p.1),
        }
    }

    fn distance(&self, other: &Point) -> i32 {
        let d = *other - *self;
        d.0.abs() + d.1.abs() + d.2.abs()
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

fn generator(input: &str) -> Vec<BTreeSet<Point>> {
    let mut r: Vec<BTreeSet<Point>> = vec![];
    for l in input.lines() {
        let len = r.len();
        match l
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(v) if v.len() == 3 => {
                r[len - 1].insert(Point(v[0], v[1], v[2]));
            }
            _ => {
                if r.is_empty() || !r[len - 1].is_empty() {
                    r.push(BTreeSet::new());
                }
            }
        }
    }
    r
}

fn intersect(s1: &BTreeSet<Point>, s2: &BTreeSet<Point>) -> Option<(BTreeSet<Point>, Point)> {
    for i in 0..24 {
        let s2: Vec<Point> = s2.iter().map(|p| p.rotate(i)).collect();
        for &r1 in s1 {
            for &r2 in &s2 {
                let o2 = r2 - r1;
                let s2 = s2.iter().map(|&p| p - o2).collect();
                if s1.intersection(&s2).count() >= 12 {
                    return Some((s2, Point::default() - o2));
                }
            }
        }
    }
    None
}

fn normalize(scanners: Vec<BTreeSet<Point>>) -> Vec<(BTreeSet<Point>, Point)> {
    let mut left = scanners
        .into_iter()
        .map(|s| {
            (
                s.clone(),
                s.iter()
                    .flat_map(|a1| {
                        s.iter()
                            .filter_map(move |a2| (a1 != a2).then(|| a1.distance(a2)))
                    })
                    .collect::<BTreeSet<_>>(),
            )
        })
        .collect::<Vec<_>>();
    let mut found = vec![(left.pop().unwrap(), Point::default())];
    while !left.is_empty() {
        let mut not_found = Vec::new();
        for (s2, d2) in left {
            match found.iter().find_map(|((s1, d1), _)| {
                if d1.intersection(&d2).count() >= 66 {
                    intersect(s1, &s2)
                } else {
                    None
                }
            }) {
                Some((s2, o)) => found.push(((s2, d2), o)),
                None => not_found.push((s2, d2)),
            }
        }
        left = not_found;
    }
    found.into_iter().map(|((s, _), o)| (s, o)).collect()
}

#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
    normalize(generator(input))
        .into_iter()
        .flat_map(|(s, _)| s)
        .collect::<BTreeSet<_>>()
        .len()
}

#[aoc(day19, part2)]
fn part2(input: &str) -> i32 {
    let points = normalize(generator(input))
        .into_iter()
        .map(|(_, o)| o)
        .collect::<Vec<_>>();
    points
        .iter()
        .enumerate()
        .flat_map(|(i, o1)| points[i + 1..].iter().map(|o2| o1.distance(o2)))
        .max()
        .unwrap()
}
