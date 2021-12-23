use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use std::{collections::BTreeMap, convert::Infallible, str::FromStr};

#[derive(Clone, Eq, Hash, PartialEq)]
struct Game {
    pos: Vec<i64>, // 0..=10 is the hallway, then first room, second room, etc.
}

impl Game {
    fn up(pos: i64) -> Option<i64> {
        if pos > 10 && pos % 4 == 3 {
            Some((pos - 11) / 4 * 2 + 2)
        } else {
            (pos > 10).then(|| pos - 1)
        }
    }

    fn down(pos: i64) -> Option<i64> {
        if pos > 0 && pos < 10 && pos % 2 == 0 {
            Some(11 + (pos - 2) / 2 * 4)
        } else {
            (pos > 10 && pos % 4 != 2).then(|| pos + 1)
        }
    }

    fn all_below(pos: i64) -> impl Iterator<Item = i64> {
        let mut pos = Some(pos);
        std::iter::from_fn(move || {
            pos = pos.and_then(Self::down);
            pos
        })
    }

    fn all_above(pos: i64) -> impl Iterator<Item = i64> {
        let mut pos = Some(pos);
        std::iter::from_fn(move || {
            pos = pos.and_then(Self::up);
            pos
        })
    }

    fn is_free(&self, pos: i64) -> bool {
        !self.pos.contains(&pos)
    }

    fn in_front_of_chamber(pos: i64) -> Option<i64> {
        ((1..10).contains(&pos) && pos % 2 == 0).then(|| pos + 9)
    }

    fn in_room(pos: i64) -> Option<usize> {
        (pos > 10).then(|| (pos as usize - 11) / 4)
    }

    fn is_over(&self) -> bool {
        self.pos
            .iter()
            .enumerate()
            .all(|(i, &p)| Self::in_room(p) == Some(i / 4))
    }

    fn destinations(&self, idx: usize) -> Vec<(Self, i64)> {
        let mut d = vec![];
        let pos = self.pos[idx];
        if let Some(room) = Self::in_room(pos) {
            let above = Self::all_above(pos).collect::<Vec<_>>();
            let can_leave = above.iter().all(|&p| self.is_free(p));
            let must_leave = Self::all_below(pos)
                .any(|p| self.idx_at(p).map(|i| i / 4 != idx / 4).unwrap_or(false));
            if (room != idx / 4 || must_leave) && can_leave {
                let h = *above.last().unwrap();
                let mut v = above.clone();
                for i in (0..h).rev() {
                    if !self.is_free(i) {
                        break;
                    }
                    v.push(i);
                    if Self::in_front_of_chamber(i).is_none() {
                        d.push(v.clone());
                    }
                }
                let mut v = above;
                for i in (h + 1..=10).take_while(|&p| self.is_free(p)) {
                    v.push(i);
                    if Self::in_front_of_chamber(i).is_none() {
                        d.push(v.clone());
                    }
                }
            }
        } else {
            let h = (idx / 4 + 1) as i64 * 2;
            let can_enter = Self::all_below(h)
                .all(|p| self.idx_at(p).map(|i| i / 4 == idx / 4).unwrap_or(true));
            if can_enter {
                let mut v = if h > pos {
                    (pos + 1..=h).collect::<Vec<_>>()
                } else {
                    (h..pos).rev().collect::<Vec<_>>()
                };
                if v.iter().all(|&p| self.is_free(p)) {
                    v.extend(Self::all_below(h).take_while(|&p| self.is_free(p)));
                    d.push(v);
                }
            }
        }
        let cost = [1, 10, 100, 1000][idx / 4];
        d.into_iter()
            .map(|t| {
                let mut game = self.clone();
                game.pos[idx] = t.last().cloned().unwrap();
                (game, cost * t.len() as i64)
            })
            .collect()
    }

    fn neighbours(&self) -> Vec<(Self, i64)> {
        (0..16).flat_map(|idx| self.destinations(idx)).collect()
    }

    fn idx_at(&self, pos: i64) -> Option<usize> {
        self.pos
            .iter()
            .find_position(|&&p| p == pos)
            .map(|(idx, _)| idx)
    }
}

impl FromStr for Game {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pos = BTreeMap::new();
        let mut c = 11;
        s.bytes().for_each(|b| {
            if (b'A'..=b'D').contains(&b) {
                pos.entry(b).or_insert_with(Vec::new).push(c);
                c += 4;
                if c == 27 {
                    c = 14;
                }
            }
        });
        Ok(Game {
            pos: pos.values().flatten().cloned().collect(),
        })
    }
}

#[aoc(day23, part1)]
fn part1(input: &str) -> i64 {
    let mut game: Game = input.parse().unwrap();
    game.pos.iter_mut().for_each(|p| {
        if *p % 4 == 2 {
            *p -= 2
        }
    });
    for i in 0..4 {
        game.pos.insert(4 * i + 2, 13 + 4 * i as i64);
        game.pos.insert(4 * i + 3, 14 + 4 * i as i64);
    }
    dijkstra(&game, |g| g.neighbours(), |g| g.is_over())
        .unwrap()
        .1
}

#[aoc(day23, part2)]
fn part2(input: &str) -> i64 {
    let mut game: Game = input.parse().unwrap();
    game.pos = vec![
        game.pos[0],
        game.pos[1],
        21,
        24,
        game.pos[2],
        game.pos[3],
        17,
        20,
        game.pos[4],
        game.pos[5],
        16,
        25,
        game.pos[6],
        game.pos[7],
        12,
        13,
    ];
    dijkstra(&game, |g| g.neighbours(), |g| g.is_over())
        .unwrap()
        .1
}
