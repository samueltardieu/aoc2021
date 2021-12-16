use crate::bits::Packet;

#[aoc(day16, part1)]
fn part1(input: &str) -> anyhow::Result<u32> {
    Ok(input.trim().parse::<Packet>()?.version_sum())
}

#[aoc(day16, part2)]
fn part2(input: &str) -> anyhow::Result<u64> {
    Ok(input.trim().parse::<Packet>()?.eval())
}
