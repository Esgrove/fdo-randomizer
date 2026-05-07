use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::Duration;

use colored::Colorize;

static AUDIO_EXTENSIONS: [&str; 6] = ["aif", "aiff", "flac", "mp3", "m4a", "wav"];

/// Calculate hash for the given list order.
pub fn get_ordering_hash(files: &Vec<PathBuf>) -> u64 {
    let mut hasher = DefaultHasher::new();
    files.hash(&mut hasher);
    hasher.finish()
}

/// Returns true if the given file is one of the supported audio file types.
pub fn is_audio_file(path: &Path) -> bool {
    path.extension().is_some_and(|ext| {
        let ext_str = ext.to_string_lossy().to_lowercase();
        AUDIO_EXTENSIONS.contains(&ext_str.as_str())
    })
}

/// Pretty-print elapsed time duration.
pub fn print_duration(elapsed: Duration) {
    let formatted_time = format!("{:.2}s", elapsed.as_secs_f64());
    println!("{}", format!("Finished in {formatted_time}").green());
}
