use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;

    let matches = find_and_print_matches(&content, &args.pattern);

    println!("Found {} matches.", matches);

    Ok(())
}

fn find_and_print_matches(content: &str, pattern: &str) -> usize {
    content
        .lines()
        .filter(|line| line.contains(pattern))
        .inspect(|line| println!("{}", line))
        .count()
}

#[test]
fn test_find_matches() {
    let matches = find_and_print_matches("ajdfau\njajauau8ru\ngt585t\nauau9ufjia\njighau58hu", "au");
    assert_eq!(matches, 4);
}
