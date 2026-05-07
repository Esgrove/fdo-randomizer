mod randomizer;
mod utils;

use std::path::PathBuf;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
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

    /// Optional number of randomized orders to generate (default is 1)
    permutations: Option<usize>,

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
    let args = Args::parse();
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

    let mut permutations = args.permutations.unwrap_or(1);
    if permutations > 99 {
        println!(
            "{}",
            format!("That's a lot of permutations ({permutations}), limiting to 99...").yellow()
        );
        permutations = 99;
    }

    randomizer::generate_unique_permutations(
        &absolute_input_path,
        absolute_output_root,
        permutations,
        args.verbose,
        args.force,
    )
}

#[cfg(test)]
mod cli_args_tests {
    use super::*;

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
}
