use anyhow::Context;
use anyhow::Result;
use colored::Colorize;
use rand::seq::SliceRandom;
use std::fs;
use std::path::{Path, PathBuf};

pub fn fdo_impro_randomizer(
    input_path: &PathBuf,
    output_root: PathBuf,
    permutations: usize,
    verbose: bool,
    overwrite_existing: bool,
) -> Result<()> {
    let mut files: Vec<PathBuf> = fs::read_dir(input_path)
        .context("Failed to read input directory")?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            if is_audio_file(&path) {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    if files.is_empty() {
        anyhow::bail!("No audio files found in: '{}'", input_path.display());
    }

    // Sort the files
    files.sort();
    files.dedup();

    let files_padding = files.len().to_string().chars().count();
    let permutations_padding = permutations.to_string().chars().count();

    if verbose {
        println!("Input files:");
        for (index, file) in files.iter().enumerate() {
            println!("{:>width$}: {}", index + 1, file.display(), width = files_padding);
        }
    }

    for number in 1..=permutations {
        let output_name = format!("FDO Impro {:0width$}", number, width = permutations_padding);
        let output_path = output_root.join(output_name);

        println!(
            "{}Copying files for permutation {number}...",
            if verbose { "\n" } else { "" }
        );
        if output_path.exists() {
            if overwrite_existing {
                println!(
                    "{}",
                    format!("Deleting existing output directory '{}'", output_path.display()).yellow()
                );
                fs::remove_dir_all(output_path.clone()).context(format!(
                    "Failed to remove existing output directory {}",
                    output_path.display()
                ))?;
            } else {
                eprintln!("Skipping already existing output dir: '{}'", output_path.display());
                continue;
            }
        }

        fs::create_dir_all(output_path.clone()).context("Failed to create output directory")?;

        let mut rng = rand::thread_rng();
        files.shuffle(&mut rng);
        while check_consecutive_tracks_from_same_artist(&files) {
            files.shuffle(&mut rng);
        }

        for (n, original_file) in files.iter().enumerate() {
            let new_file_name = format!(
                "{:0width$} FDO impro - {}",
                n + 1,
                original_file.file_name().unwrap().to_str().unwrap(),
                width = files.len().to_string().len()
            );
            let new_file = output_path.join(new_file_name);
            if verbose {
                println!("Copying to: {}", new_file.display());
            }
            fs::copy(original_file, new_file).context("Failed to copy file")?;
        }
    }

    Ok(())
}

fn check_consecutive_tracks_from_same_artist(tracks: &Vec<PathBuf>) -> bool {
    if tracks.len() < 2 {
        return false;
    }
    tracks
        .iter()
        .filter_map(|path| path.file_stem()?.to_str())
        .map(|s| s.split(" - ").next().unwrap_or(""))
        .collect::<Vec<_>>()
        .windows(2)
        .any(|pair| match pair {
            [previous, current] => previous == current,
            _ => false,
        })
}

fn is_audio_file(path: &Path) -> bool {
    let audio_extensions = ["aif", "aiff", "wav", "flac", "mp3", "m4a"];
    match path.extension() {
        Some(ext) => {
            let ext_str = ext.to_string_lossy().to_lowercase();
            audio_extensions.contains(&ext_str.as_str())
        }
        None => false,
    }
}
