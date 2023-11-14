use anyhow::Context;
use anyhow::Result;
use colored::Colorize;
use rand::seq::SliceRandom;
use std::fs;
use std::path::{Path, PathBuf};

use chrono;
use std::time::{Duration, Instant};

/// Generate randomized play orders for the audio files from the given input directory.
/// Copies audio files from input folder to new folders with numbered names in the created random order.
/// The permutation parameter controls how many folders to generate.
pub fn fdo_impro_randomizer(
    input_path: &PathBuf,
    output_root: PathBuf,
    permutations: usize,
    verbose: bool,
    overwrite_existing: bool,
) -> Result<()> {
    println!(
        "Generating {} randomized audio file permutations to: {}\n",
        permutations,
        output_root.display()
    );

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

    // Sort the files and remove duplicates
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

    fs::create_dir_all(output_root.clone()).context("Failed to create output root directory")?;

    let start_time = Instant::now();
    for number in 1..=permutations {
        let output_name = format!("FDO Impro {:0width$}", number, width = permutations_padding);
        let output_path = output_root.join(output_name);

        println!(
            "{}Copying files for permutation {number}...",
            if verbose { "\n" } else { "" }
        );
        if output_path.exists() {
            let absolute_output_path =
                fs::canonicalize(output_path.clone()).context("Failed to get absolute path for output dir")?;
            if overwrite_existing {
                println!(
                    "{}",
                    format!(
                        "Deleting existing output directory '{}'",
                        absolute_output_path.display()
                    )
                    .yellow()
                );
                fs::remove_dir_all(absolute_output_path.clone()).context(format!(
                    "Failed to remove existing output directory {}",
                    absolute_output_path.display()
                ))?;
            } else {
                eprintln!(
                    "Skipping already existing output dir: '{}'",
                    absolute_output_path.display()
                );
                continue;
            }
        }

        fs::create_dir_all(output_path.clone()).context("Failed to create output directory")?;

        let absolute_output_path =
            fs::canonicalize(output_path).context("Failed to get absolute path for output dir")?;

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
            let new_file = absolute_output_path.join(new_file_name);
            if verbose {
                println!("Copying to: {}", new_file.display());
            }
            fs::copy(original_file, new_file).context("Failed to copy file")?;
        }
    }

    let elapsed = start_time.elapsed();
    print_duration(elapsed);

    Ok(())
}

/// Pretty-print elapsed time duration
fn print_duration(elapsed: Duration) {
    let duration = chrono::Duration::seconds(elapsed.as_secs() as i64)
        + chrono::Duration::milliseconds(elapsed.subsec_millis() as i64);

    let hours = duration.num_hours();
    let minutes = if hours > 0 {
        duration.num_minutes() % 60
    } else {
        duration.num_minutes()
    };
    let seconds = if minutes > 0 {
        duration.num_seconds() % 60
    } else {
        duration.num_seconds()
    };
    let milliseconds = if seconds > 0 {
        duration.num_milliseconds() % 60
    } else {
        duration.num_milliseconds()
    };

    let formatted_time = if hours > 0 {
        format!("{:02}h:{:02}m:{:02}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{:02}m:{:02}s", minutes, seconds)
    } else if seconds > 0 {
        format!("{:02}s:{:02}ms", seconds, milliseconds)
    } else {
        format!("{:02}ms", milliseconds)
    };
    if !formatted_time.is_empty() {
        println!("{} ({:?})", format!("Finished in: {}", formatted_time).green(), elapsed);
    }
}

/// Returns false when there are no consecutive files with the same artist name.
/// This assumes all files are named in the format: <artist> - <title>
fn check_consecutive_tracks_from_same_artist(tracks: &Vec<PathBuf>) -> bool {
    if tracks.len() < 2 {
        return false;
    }
    tracks
        .iter()
        .filter_map(|path| path.file_stem()?.to_str())
        .map(|s| s.split(" - ").next().unwrap_or(s))
        .collect::<Vec<_>>()
        .windows(2)
        .any(|pair| match pair {
            [previous, current] => previous == current,
            _ => false,
        })
}

/// Returns true if the given file is one of the supported audio file types
fn is_audio_file(path: &Path) -> bool {
    let audio_extensions = ["aif", "aiff", "flac", "mp3", "m4a", "wav"];
    match path.extension() {
        Some(ext) => {
            let ext_str = ext.to_string_lossy().to_lowercase();
            audio_extensions.contains(&ext_str.as_str())
        }
        None => false,
    }
}
