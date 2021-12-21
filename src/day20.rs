use itertools::iproduct;
use pathfinding::prelude::Matrix;

fn generator(input: &str) -> (Vec<u8>, Matrix<u8>) {
    let mut lines = input.lines();
    (
        lines.next().unwrap().as_bytes().to_vec(),
        lines.skip(1).map(|s| s.bytes()).collect(),
    )
}

fn decode(image: &Matrix<u8>, decoder: &[u8], odd: bool) -> Matrix<u8> {
    let default = odd.then(|| decoder[0]).unwrap_or(b'.');
    (0..image.rows + 2)
        .map(|y| {
            (0..image.columns + 2).map(move |x| {
                decoder[iproduct!(0..=2, 0..=2).fold(0usize, |a, (dy, dx)| {
                    (a << 1)
                        | (image
                            .get(((y + dy).wrapping_sub(2), (x + dx).wrapping_sub(2)))
                            .unwrap_or(&default)
                            == &b'#') as usize
                })]
            })
        })
        .collect()
}

#[aoc(day20, part1)]
fn part1(input: &str) -> usize {
    let (decoder, image) = generator(input);
    (0..2)
        .fold(image, |g, i| decode(&g, &decoder, i % 2 == 1))
        .values()
        .filter(|&&b| b == b'#')
        .count()
}

#[aoc(day20, part2)]
fn part2(input: &str) -> usize {
    let (decoder, image) = generator(input);
    (0..50)
        .fold(image, |g, i| decode(&g, &decoder, i % 2 == 1))
        .values()
        .filter(|&&b| b == b'#')
        .count()
}
