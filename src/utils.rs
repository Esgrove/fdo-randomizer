use std::env;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};
use clap::Command;
use clap_complete::Shell;
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

/// Resolve the input directory to an absolute path.
///
/// If `input_path` is `None` or empty, the current working directory is used.
/// The resolved path must exist and be a directory.
///
/// # Errors
/// Returns an error if:
/// - The current working directory cannot be determined
/// - The input path does not exist or is not accessible
/// - The input path is not a directory
/// - Path canonicalization fails
pub fn resolve_input_path(input_path: Option<&Path>) -> Result<PathBuf> {
    let input_path = match input_path {
        Some(path) if !path.as_os_str().is_empty() => path.to_path_buf(),
        _ => env::current_dir().context("Failed to get current working directory")?,
    };

    if !input_path.exists() {
        anyhow::bail!(
            "Input directory does not exist or is not accessible: '{}'",
            input_path.display()
        );
    }
    if !input_path.is_dir() {
        anyhow::bail!("Input path is not a directory: '{}'", input_path.display());
    }

    let absolute_input_path = dunce::canonicalize(&input_path)?;
    Ok(absolute_input_path)
}

/// Resolve the output root directory path.
///
/// If `output_path` is `None`, the parent directory of `input_path` is used.
/// Relative output paths are resolved against the current working directory.
///
/// # Errors
/// Returns an error if:
/// - The input path has no parent directory when no output path is provided
/// - The current working directory cannot be determined
/// - The output path is empty
pub fn resolve_output_root(output_path: Option<&Path>, input_path: &Path) -> Result<PathBuf> {
    match output_path {
        None => input_path
            .parent()
            .context("Input path has no parent directory")
            .map(Path::to_path_buf),
        Some(output_path) => {
            if output_path.as_os_str().is_empty() {
                anyhow::bail!("empty output path");
            }
            if output_path.is_absolute() {
                Ok(output_path.to_path_buf())
            } else {
                let current_dir = dunce::canonicalize(env::current_dir().context("Failed to get current directory")?)?;
                Ok(current_dir.join(output_path))
            }
        }
    }
}

/// Determine the appropriate directory for storing shell completions.
///
/// First checks if the user-specific directory exists,
/// then checks for the global directory.
/// If neither exist, creates and uses the user-specific dir.
fn get_shell_completion_dir(shell: Shell, name: &str) -> Result<PathBuf> {
    let home = dirs::home_dir().expect("Failed to get home directory");

    if shell == Shell::Zsh {
        let oh_my_zsh_plugins = home.join(".oh-my-zsh/custom/plugins");
        if oh_my_zsh_plugins.exists() {
            let plugin_dir = oh_my_zsh_plugins.join(name);
            std::fs::create_dir_all(&plugin_dir)?;
            return Ok(plugin_dir);
        }
    }

    let user_dir = match shell {
        Shell::PowerShell => {
            if cfg!(windows) {
                home.join(r"Documents\PowerShell\completions")
            } else {
                home.join(".config/powershell/completions")
            }
        }
        Shell::Bash => home.join(".bash_completion.d"),
        Shell::Elvish => home.join(".elvish"),
        Shell::Fish => home.join(".config/fish/completions"),
        Shell::Zsh => home.join(".zsh/completions"),
        _ => anyhow::bail!("Unsupported shell"),
    };

    if user_dir.exists() {
        return Ok(user_dir);
    }

    let global_dir = match shell {
        Shell::PowerShell => {
            if cfg!(windows) {
                home.join(r"Documents\PowerShell\completions")
            } else {
                home.join(".config/powershell/completions")
            }
        }
        Shell::Bash => PathBuf::from("/etc/bash_completion.d"),
        Shell::Fish => PathBuf::from("/usr/share/fish/completions"),
        Shell::Zsh => PathBuf::from("/usr/share/zsh/site-functions"),
        _ => anyhow::bail!("Unsupported shell"),
    };

    if global_dir.exists() {
        return Ok(global_dir);
    }

    std::fs::create_dir_all(&user_dir)?;
    Ok(user_dir)
}

/// Generate a shell completion script for the given shell.
///
/// # Errors
/// Returns an error if:
/// - The shell completion directory cannot be determined or created
/// - The completion file cannot be generated or written
pub fn generate_shell_completion(
    shell: Shell,
    mut command: Command,
    install: bool,
    verbose: bool,
    command_name: &str,
) -> Result<()> {
    if install {
        let out_dir = get_shell_completion_dir(shell, command_name)?;
        let path = clap_complete::generate_to(shell, &mut command, command_name, out_dir)?;
        if verbose {
            println!("Completion file generated to: {}", path.display());
        }
    } else {
        clap_complete::generate(shell, &mut command, command_name, &mut std::io::stdout());
    }
    Ok(())
}
