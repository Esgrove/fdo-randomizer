use std::cmp::min;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Result};
use colored::Colorize;
use rand::seq::SliceRandom;

static AUDIO_EXTENSIONS: [&str; 6] = ["aif", "aiff", "flac", "mp3", "m4a", "wav"];

/// Generate randomized play orders for the audio files from the given input directory.
/// Copies audio files from input folder to new folders with numbered names in the created random order.
/// The permutation parameter controls how many folders to generate.
pub fn generate_unique_permutations(
    input_path: &PathBuf,
    output_root: PathBuf,
    permutations: usize,
    verbose: bool,
    overwrite_existing: bool,
) -> Result<()> {
    fs::create_dir_all(&output_root).context("Failed to create output root directory")?;
    let absolute_output_root =
        dunce::canonicalize(output_root).context("Failed to get absolute path for output root directory")?;

    let mut files = gather_audio_files(input_path)?;
    if files.is_empty() {
        anyhow::bail!("No audio files found in: '{}'", input_path.display());
    }

    // Might run into problems with uniqueness if there is only a small number of files
    let num_permutations: usize = if files.len() <= 20 {
        // Calculate factorial for number of files
        let max_unique_permutations: usize = (1..=files.len()).product();
        if permutations > max_unique_permutations {
            println!(
                "{}",
                format!("Limiting permutations to {max_unique_permutations} possible unique orderings!").red()
            );
        }
        min(permutations, max_unique_permutations)
    } else {
        permutations
    };

    println!(
        "Generating {num_permutations} randomized audio file permutations of {} tracks to: {}\n",
        files.len(),
        absolute_output_root.display()
    );

    let files_padding = files.len().to_string().chars().count();
    if verbose {
        println!("Input files:");
        for (index, file) in files.iter().enumerate() {
            println!("{:>files_padding$}: {}", index + 1, file.display());
        }
    }

    // Keep track of generated randomized orderings
    let mut orderings: HashSet<u64> = HashSet::new();
    let permutations_padding = num_permutations.to_string().chars().count();
    let start_time = Instant::now();
    for number in 1..=num_permutations {
        let output_name = format!("FDO Impro {number:0permutations_padding$}");
        let output_path = absolute_output_root.join(output_name.clone());

        println!(
            "{}Copying files for {}...",
            if verbose { "\n" } else { "" },
            output_name
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

        get_unique_file_ordering(&mut files, &mut orderings)?;

        fs::create_dir_all(output_path.clone()).context("Failed to create output directory")?;
        copy_files_with_numbered_naming(&files, &output_path, verbose)?;
    }

    let elapsed = start_time.elapsed();
    print_duration(elapsed)?;

    Ok(())
}

/// Copy files to given new folder with a running index added to the start of the filename.
fn copy_files_with_numbered_naming(files: &[PathBuf], output_path: &Path, verbose: bool) -> Result<()> {
    for (index, original_file) in files.iter().enumerate() {
        let file_name = original_file
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .ok_or_else(|| anyhow!("Invalid file name in path: {:?}", original_file))?;

        let width = files.len().to_string().len();
        let number = index + 1;
        let new_file_name = format!("{number:0width$} FDO impro - {file_name}");
        let new_file = output_path.join(new_file_name);

        if verbose {
            println!("  Copying to: {}", new_file.display());
        }

        fs::copy(original_file, &new_file).context("Failed to copy file")?;
    }
    Ok(())
}

/// Get a list of all the audio files present in the input path.
/// Note: non-recursive so only looks at the files directly in the given directory.
fn gather_audio_files(input_path: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = fs::read_dir(input_path)
        .context("Failed to read input directory")?
        .filter_map(std::result::Result::ok)
        .filter_map(|entry| {
            let path = entry.path();
            if is_audio_file(&path) {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    // Sort the files and remove duplicates
    files.sort();
    files.dedup();

    Ok(files)
}

/// Returns true if there are consecutive files with the same artist name.
/// This assumes all files are named in the format: <artist> - <title>
fn check_consecutive_tracks_from_same_artist(tracks: &[PathBuf]) -> bool {
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

/// Keep shuffling song order until there are no consecutive tracks from the same artists
/// and the order is different from all previous orderings.
fn get_unique_file_ordering(files: &mut Vec<PathBuf>, orderings: &mut HashSet<u64>) -> Result<()> {
    let mut rng = rand::thread_rng();
    files.shuffle(&mut rng);
    let mut hash = get_ordering_hash(files);
    let mut tries: usize = 0;
    while check_consecutive_tracks_from_same_artist(files) || orderings.contains(&hash) {
        files.shuffle(&mut rng);
        hash = get_ordering_hash(files);
        tries += 1;
        if tries > 1000 {
            anyhow::bail!("Failed to create an unique random order, quitting")
        }
    }
    hash = get_ordering_hash(files);
    orderings.insert(hash);
    Ok(())
}

/// Calculate hash for the given list order
fn get_ordering_hash(files: &Vec<PathBuf>) -> u64 {
    let mut hasher = DefaultHasher::new();
    files.hash(&mut hasher);
    hasher.finish()
}

/// Returns true if the given file is one of the supported audio file types
fn is_audio_file(path: &Path) -> bool {
    path.extension().map_or(false, |ext| {
        let ext_str = ext.to_string_lossy().to_lowercase();
        AUDIO_EXTENSIONS.contains(&ext_str.as_str())
    })
}

/// Pretty-print elapsed time duration
fn print_duration(elapsed: Duration) -> Result<()> {
    #[allow(clippy::cast_possible_wrap)]
    let duration = chrono::TimeDelta::try_seconds(elapsed.as_secs() as i64)
        .ok_or_else(|| anyhow!("Failed to create duration from seconds"))?
        + chrono::TimeDelta::try_milliseconds(i64::from(elapsed.subsec_millis()))
            .ok_or_else(|| anyhow!("Failed to create duration from milliseconds"))?;

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
        duration.num_milliseconds() % 1000
    } else {
        duration.num_milliseconds()
    };

    let formatted_time = if hours > 0 {
        format!("{hours:02}h:{minutes:02}m:{seconds:02}s")
    } else if minutes > 0 {
        format!("{minutes:02}m:{seconds:02}s")
    } else if seconds > 0 {
        format!("{seconds:02}s:{milliseconds:02}ms")
    } else {
        format!("{milliseconds:02}ms")
    };

    if !formatted_time.is_empty() {
        println!("{} ({:?})", format!("Finished in {formatted_time}").green(), elapsed);
    }

    Ok(())
}
