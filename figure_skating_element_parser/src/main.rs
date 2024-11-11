use anyhow::{Context, Result};
use figure_skating_element_parser::*;
use std::{env, fs};

fn show_usage() {
    println!(
        "Figure Skating Element Parser CLI - Commands:\n\
         cargo run <FILE_PATH>     Analyze a file with skating elements and display parsed details.\n\
         cargo run -- --help       Display usage information.\n\
         cargo run -- --about      Show project and author information."
    );
}

fn display_credits() {
    println!("Figure Skating Element Parser by Anastasiia Chyzhova");
}

fn analyze_file(file_path: &str) -> Result<()> {
    let file_content = fs::read_to_string(file_path)
        .with_context(|| format!("Could not read file: {}", file_path))?;

    let parsed_elements =
        parse_elements(&file_content).context("Parsing skating elements failed")?;

    println!("Parsed Skating Elements:");
    parsed_elements
        .iter()
        .for_each(|element| println!("{:#?}", element));

    let total_value: f32 = parsed_elements
        .iter()
        .map(|element| element.base_value)
        .sum();
    println!("Base sum of elements: {:.1}", total_value);

    Ok(())
}

fn main() -> Result<()> {
    match env::args().nth(1).as_deref() {
        Some("--help") => show_usage(),
        Some("--about") => display_credits(),
        Some(file_path) => analyze_file(file_path)?,
        None => show_usage(),
    }

    Ok(())
}
