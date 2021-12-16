use anyhow::bail;
use aoc2021::runners;
use chrono::{Datelike, Duration};
use itertools::Itertools;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "AoC 2021", about = "Advent Of Code 2021")]
struct Opts {
    #[structopt(short, long, help = "Run all days")]
    all: bool,

    #[structopt(short, long, help = "Use a specific day")]
    day: Option<usize>,

    #[structopt(short, long, help = "Show timing information")]
    timing: bool,

    #[structopt(short, long, help = "Use alternate input (file or string)")]
    input: Option<String>,
}

fn pretty_duration(duration: Duration) -> String {
    if duration < Duration::microseconds(1) {
        format!("{} ns", duration.num_nanoseconds().unwrap())
    } else if duration < Duration::milliseconds(1) {
        format!(
            "{:.2} µs",
            duration.num_nanoseconds().unwrap() as f32 / 1000.0
        )
    } else if duration < Duration::seconds(1) {
        format!(
            "{:.2} ms",
            duration.num_microseconds().unwrap() as f32 / 1000.0
        )
    } else {
        format!("{:.2} s", duration.num_milliseconds() as f32 / 1000.0)
    }
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();
    if opts.day.is_some() && opts.all {
        bail!("--all and --day are not compatible");
    }
    if opts.input.is_some() && opts.all {
        bail!("--all and --input are not compatible");
    }
    unsafe {
        aoc2021::OVERRIDE_INPUT = opts.input;
    }
    let current_day = opts.day.unwrap_or(chrono::Utc::now().day() as usize);
    aoc2021::runners::register();
    let mut runners = runners::RUNNERS.lock().unwrap();
    let keys = runners.keys().cloned().collect::<Vec<_>>();
    for (day, part) in keys {
        if day == current_day || opts.all {
            for (version, runner) in runners.remove(&(day, part)).unwrap() {
                let before = chrono::Utc::now();
                let result = runner();
                let after = chrono::Utc::now();
                let version = version
                    .clone()
                    .map(|v| format!(" — {}", v))
                    .unwrap_or_else(String::new);
                let elapsed = if opts.timing {
                    format!(" ({})", pretty_duration(after - before))
                } else {
                    String::new()
                };
                let header = format!("Day {} - part {}{}: ", day, part, version);
                let sep = format!("\n{}", " ".repeat(header.len()));
                let result = match result {
                    Ok(e) => e.lines().join(&sep),
                    Err(e) => format!("<error: {:?}>", e),
                };
                println!("{}{}{}", header, result, elapsed);
            }
        }
    }
    Ok(())
}
