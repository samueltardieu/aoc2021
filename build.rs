use regex::Regex;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

fn output(refs: &[String]) -> anyhow::Result<()> {
    let outdir = std::env::var("OUT_DIR")?;
    let mut outfile = PathBuf::from(outdir);
    outfile.push("register.rs");
    let mut fd = File::create(outfile)?;
    writeln!(fd, "pub fn register() {{")?;
    writeln!(fd, "  let _ = {};", refs.join("+"))?;
    writeln!(fd, "}}")?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let aoc = Regex::new(r#"#\[aoc\(day(\d+),\s*part(\d+),?\s*(\S*)\)\]"#).unwrap();
    let mut refs = Vec::new();
    for file in fs::read_dir("src")? {
        let file = file?;
        if !file
            .file_name()
            .into_string()
            .map(|s| s.ends_with(".rs"))
            .unwrap_or(false)
        {
            continue;
        }
        for l in BufReader::new(File::open(file.path())?).lines() {
            if let Some(m) = aoc.captures(&l?) {
                let day = &m[1];
                let part = &m[2];
                let version = if m[3].is_empty() { "none" } else { &m[3] };
                refs.push(format!(
                    "*crate::{}::RUNNER_{}_{}_{}",
                    file.file_name()
                        .into_string()
                        .unwrap()
                        .strip_suffix(".rs")
                        .unwrap(),
                    day,
                    part,
                    version
                ));
            }
        }
    }
    output(&refs)?;
    Ok(())
}
