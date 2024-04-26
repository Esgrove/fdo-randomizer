# FDO Impro Randomizer

Generate randomized play orders from a given folder of audio files.

Created for the [Finnish Dance Organization](https://fdo.fi/) "Improvisation" competition category music.

## Rust version

```console
Usage: fdo-randomizer [OPTIONS] <INPUT_DIR> [PERMUTATIONS]

Arguments:
  <INPUT_DIR>     Input directory with audio files to randomize
  [PERMUTATIONS]  Optional number of randomized orders to generate (default is 1)

Options:
  -o, --output <PATH>  Optional output root path (default is input path parent dir)
  -f, --force          Overwrite existing output directories
  -v, --verbose        Verbose output
  -h, --help           Print help
  -V, --version        Print version
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

cargo run --release -- "input dir" 10
```

Debug errors:

```shell
RUST_BACKTRACE=1 cargo run -- FOLDER 8 --force --output "$HOME/Downloads/IMPRO"
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
