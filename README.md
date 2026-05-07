# FDO Impro Randomizer

Generate randomized play orders from a given folder of audio files.

Created for the [Finnish Dance Organization](https://fdo.fi/) "Improvisation" competition category music.

## Rust version

```console
Usage: fdo-randomizer [OPTIONS] [INPUT_DIR] [PERMUTATIONS] [COMMAND]

Commands:
  completion  Generate shell completion script
  help        Print this message or the help of the given subcommand(s)

Arguments:
  [INPUT_DIR]     Input directory with audio files to randomize
  [PERMUTATIONS]  Number of randomized orders to generate

Options:
  -p, --permutations <NUM>  Number of randomized orders to generate [default: 1]
  -o, --output <PATH>       Optional output root path (default is input path parent dir)
  -f, --force               Overwrite existing output directories
  -v, --verbose             Verbose output
  -h, --help                Print help
  -V, --version             Print version
```

## Shell completions

The Rust binary supports shell completion generation via the `completion` subcommand.
The install script also installs completions for the appropriate shells on your platform:

- Windows: bash and powershell
- macOS: zsh
- Linux: zsh and bash

### Install binary and completions

```shell
./install.sh
```

### Install completions manually

```shell
./completions.sh
```

### Generate manually for a single shell

```shell
fdo-randomizer completion bash
fdo-randomizer completion zsh

fdo-randomizer completion zsh --install
fdo-randomizer completion powershell --install
```

### Build and run

[Install Rust](https://www.rust-lang.org/tools/install) and then:

```shell
./build.sh
./fdo-randomizer --help
```

Or directly with:

```shell
cargo run --release -- --help

cargo run --release -- "input dir" --permutations 10

# Positional permutations
cargo run --release -- "input dir" 10
```

Debug errors:

```shell
RUST_BACKTRACE=1 cargo run -- FOLDER --permutations 8 --force --output "$HOME/Downloads/IMPRO"
```

## Python version

> [!NOTE]
> You should probably be using the newer and better Rust version instead.

### Dependencies

- Python 3.11+
- click
- colorama

Project is handled by [Poetry](https://github.com/python-poetry/poetry).

### Run

```shell
poetry run python randomizer/randomizer.py <input_dir> [num_permutations]
```
