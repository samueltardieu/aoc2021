# AOC quick'n dirty macro set

Those are implemented using quick'n dirty procedural macros and libraries.

## Running the program

Use `cargo run --release -- [OPTIONS]`.

Options are:

- `--day N` (or `-d N`): run day `N` instead of the current day of month
- `--all` (or `-a`): run all days
- `--input FILE` (or `-i FILE`): use a specific input file instead of `input/dayN.txt`
- `--timing` (or `-t`): include timing information

## Functions

Add a module for every day in `src/lib.rs`. Modules must contain functions such as (here `src/day1.rs`):

```rust
#[aoc(day1, part1)]
fn part1(input: &[u32]) -> usize {
    input.windows(2).filter(|v| v[1] > v[0]).count()
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> usize {
    input.windows(4).filter(|v| v[3] > v[0]).count()
}
```

Alternate versions are also supported:

```rust
#[aoc(day1, part2, alternate_version)]
fn part2_alternate_version(input: &[u32]) -> usize {
    use itertools::Itertools;
    part1(
        &input
            .iter()
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .collect::<Vec<_>>(),
    )
}
```

## Arguments

Functions arguments can be:

- nothing
- `&str`: the content of the input file;
- `&[u8]`: the content of the input file;
- `&[T] where T: FromStr`: the content of each line of the input file parsed through `FromStr::from_str()`.

## Return value

The return value must implement `Display` and can be wrapped in a `Result` (the string `Result<` is looked up to detect this).
