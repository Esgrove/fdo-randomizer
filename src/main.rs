mod randomizer;

use std::path::Path;
use std::{env, fs};

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;

use crate::randomizer::generate_unique_permutations;

#[derive(Parser)]
#[command(author, about, version, arg_required_else_help = true)]
struct Args {
    /// Input directory with audio files to randomize
    input_dir: String,

    /// Optional number of randomized orders to generate (default is 1)
    permutations: Option<usize>,

    /// Optional output root path (default is input path parent dir)
    #[arg(short, long = "output", name = "PATH")]
    output_path: Option<String>,

    /// Overwrite existing output directories
    #[arg(short, long)]
    force: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input_path = args.input_dir.trim();
    if input_path.is_empty() {
        anyhow::bail!("empty input path");
    }
    let filepath = Path::new(input_path);
    if !filepath.is_dir() {
        anyhow::bail!(
            "Input directory does not exist or is not accessible: '{}'",
            filepath.display()
        );
    }
    let absolute_input_path = fs::canonicalize(filepath)?;

    println!("Input path: {}", absolute_input_path.display());
    let mut permutations = args.permutations.unwrap_or(1);

    if permutations > 99 {
        println!(
            "{}",
            format!("That's a lot of permutations ({permutations}), limiting to 99...").yellow()
        );
        permutations = 99;
    }

    let absolute_output_root = match args.output_path {
        None => absolute_input_path
            .parent()
            .context("Input path has no parent directory")?
            .to_path_buf(),
        Some(path) => {
            let output_path = path.trim();
            if output_path.is_empty() {
                anyhow::bail!("empty output path");
            }
            let path = Path::new(output_path);
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                let current_dir = fs::canonicalize(env::current_dir().context("Failed to get current directory")?)?;
                current_dir.join(path)
            }
        }
    };

    generate_unique_permutations(
        &absolute_input_path,
        absolute_output_root,
        permutations,
        args.verbose,
        args.force,
    )
}
