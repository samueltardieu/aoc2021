use regex::Regex;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

fn output(runners: &[String]) -> anyhow::Result<()> {
    let outdir = std::env::var("OUT_DIR")?;
    let mut outfile = PathBuf::from(outdir);
    outfile.push("register.rs");
    let mut fd = File::create(outfile)?;
    writeln!(fd, "pub fn register() {{")?;
    for runner in runners {
        writeln!(fd, "    {}", runner)?;
    }
    writeln!(fd, "}}")?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let attr_re = Regex::new(r#"#\[aoc\(day(\d+),\s*part(\d+),?(.*)\)\]"#).unwrap();
    let version_re = Regex::new(r"^\w+$").unwrap();
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
            if let Some(m) = attr_re.captures(&l?) {
                let day = &m[1];
                let part = &m[2];
                let version = m[3]
                    .split(',')
                    .map(|s| s.trim())
                    .find(|s| version_re.is_match(s));
                let (version, extension) = match version {
                    Some(v) => (format!("Some({})", v), v),
                    None => (String::from("None"), "none"),
                };
                refs.push(format!(
                    "crate::runners::register_runner({1}, {2}, {4}, crate::{0}::runner_{1}_{2}_{3});",
                    file.file_name()
                        .into_string()
                        .unwrap()
                        .strip_suffix(".rs")
                        .unwrap(),
                    day,
                    part,
                    extension,
                    version,
                ));
            }
        }
    }
    output(&refs)?;
    Ok(())
}
