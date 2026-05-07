mod randomizer;
mod utils;

use std::path::PathBuf;

use anyhow::Result;
use clap::parser::ValueSource;
use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
use clap_complete::Shell;
use colored::Colorize;

#[derive(Parser)]
#[command(author, about, version, arg_required_else_help = true, name = env!("CARGO_BIN_NAME"))]
struct Args {
    #[command(subcommand)]
    command: Option<CliCommand>,

    /// Input directory with audio files to randomize
    #[arg(value_hint = clap::ValueHint::DirPath)]
    input_dir: Option<PathBuf>,

    /// Number of randomized orders to generate
    #[arg(short, long, value_name = "NUM", default_value_t = 1)]
    permutations: usize,

    /// Number of randomized orders to generate
    #[arg(value_name = "PERMUTATIONS")]
    permutations_positional: Option<usize>,

    /// Optional output root path (default is input path parent dir)
    #[arg(short, long = "output", name = "PATH", value_hint = clap::ValueHint::DirPath)]
    output_path: Option<PathBuf>,

    /// Overwrite existing output directories
    #[arg(short, long)]
    force: bool,

    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum CliCommand {
    /// Generate shell completion script
    #[command(name = "completion")]
    Completion {
        /// Shell to generate completion for
        #[arg(value_enum)]
        shell: Shell,

        /// Install completion script to the shell's completion directory
        #[arg(short = 'I', long)]
        install: bool,
    },
}

fn main() -> Result<()> {
    let command = Args::command();
    let matches = command.get_matches();
    let permutations_value_source = matches.value_source("permutations");
    let args = Args::from_arg_matches(&matches)?;

    if let Some(CliCommand::Completion { shell, install }) = &args.command {
        return utils::generate_shell_completion(
            *shell,
            Args::command(),
            *install,
            args.verbose,
            env!("CARGO_BIN_NAME"),
        );
    }

    let absolute_input_path = utils::resolve_input_path(args.input_dir.as_deref())?;
    let absolute_output_root = utils::resolve_output_root(args.output_path.as_deref(), &absolute_input_path)?;

    let permutations = resolve_permutations(
        args.permutations,
        args.permutations_positional,
        permutations_value_source,
    )?;

    randomizer::generate_unique_permutations(
        &absolute_input_path,
        absolute_output_root,
        permutations,
        args.verbose,
        args.force,
    )
}

/// Resolve the effective permutations count from the preferred flag and positional argument.
///
/// The named `-p` / `--permutations` argument is preferred for new usage.
/// The positional argument is kept for backwards compatibility.
/// Values above 99 are limited to 99.
///
/// # Errors
/// Returns an error if both the named argument and legacy positional argument are provided.
fn resolve_permutations(
    permutations: usize,
    permutations_compat: Option<usize>,
    permutations_value_source: Option<ValueSource>,
) -> Result<usize> {
    let resolved_permutations = if let Some(compat_value) = permutations_compat {
        if permutations_value_source == Some(ValueSource::CommandLine) {
            anyhow::bail!("Cannot use both positional permutations and -p | --permutations arg");
        }
        compat_value
    } else {
        permutations
    };

    if resolved_permutations > 99 {
        println!(
            "{}",
            format!("That's a lot of permutations ({resolved_permutations}), limiting to 99...").yellow()
        );
        Ok(99)
    } else {
        Ok(resolved_permutations)
    }
}

#[cfg(test)]
mod cli_args_tests {
    use super::*;

    fn parse_args(arguments: &[&str]) -> (Args, Option<ValueSource>) {
        let command = Args::command();
        let matches = command.try_get_matches_from(arguments).expect("should parse");
        let permutations_value_source = matches.value_source("permutations");
        let args = Args::from_arg_matches(&matches).expect("should convert matches");
        (args, permutations_value_source)
    }

    #[test]
    fn parses_completion_with_install() {
        let args = Args::try_parse_from(["fdo-randomizer", "completion", "bash", "-I"]).expect("should parse");
        match args.command {
            Some(CliCommand::Completion { shell, install }) => {
                assert_eq!(shell, Shell::Bash);
                assert!(install);
            }
            _ => panic!("Expected Completion command"),
        }
    }

    #[test]
    fn supports_named_permutations_argument() {
        let (args, permutations_value_source) = parse_args(&["fdo-randomizer", "music", "-p", "5"]);
        let permutations = resolve_permutations(
            args.permutations,
            args.permutations_positional,
            permutations_value_source,
        )
        .expect("should resolve permutations");

        assert_eq!(args.input_dir, Some(PathBuf::from("music")));
        assert_eq!(permutations, 5);
    }

    #[test]
    fn supports_legacy_positional_permutations_argument() {
        let (args, permutations_value_source) = parse_args(&["fdo-randomizer", "music", "5"]);
        let permutations = resolve_permutations(
            args.permutations,
            args.permutations_positional,
            permutations_value_source,
        )
        .expect("should resolve permutations");

        assert_eq!(args.input_dir, Some(PathBuf::from("music")));
        assert_eq!(args.permutations_positional, Some(5));
        assert_eq!(permutations, 5);
    }

    #[test]
    fn uses_default_named_permutations_value() {
        let (args, permutations_value_source) = parse_args(&["fdo-randomizer", "music"]);
        let permutations = resolve_permutations(
            args.permutations,
            args.permutations_positional,
            permutations_value_source,
        )
        .expect("should resolve permutations");

        assert_eq!(permutations, 1);
    }

    #[test]
    fn rejects_mixing_named_and_legacy_positional_permutations() {
        let (args, permutations_value_source) = parse_args(&["fdo-randomizer", "music", "5", "-p", "2"]);
        let result = resolve_permutations(
            args.permutations,
            args.permutations_positional,
            permutations_value_source,
        );

        assert!(result.is_err());
    }

    #[test]
    fn limits_permutations_to_ninety_nine() {
        let (args, permutations_value_source) = parse_args(&["fdo-randomizer", "music", "-p", "120"]);
        let permutations = resolve_permutations(
            args.permutations,
            args.permutations_positional,
            permutations_value_source,
        )
        .expect("should resolve permutations");

        assert_eq!(permutations, 99);
    }
}
