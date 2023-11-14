# FDO Impro Randomizer

Generate randomized play orders from a given folder of audio files.

Created for the [Finnish Dance Organization](https://fdo.fi/) "Improvisation" competition category music.

## Python version

> [!NOTE]  
> You should probably be using the newer and better Rust version instead.

### Dependencies

- Python 3.11+ & Poetry
- Click
- colorama

### Run

```shell
python3 randomizer.py <input_dir> [num_permutations]
```

## Rust version

```
Usage: fdo-randomizer [OPTIONS] <INPUT_DIR> [PERMUTATIONS]

Arguments:
  <INPUT_DIR>     Input directory with audio files to randomize
  [PERMUTATIONS]  Optional number of randomized orders to generate (default is 1)

Options:
  -o, --output <OUTPUT_PATH>  Optional output root path (default is input path parent dir)
  -f, --force                 Overwrite existing output directories
  -v, --verbose               Verbose output
  -h, --help                  Print help
  -V, --version               Print version
```

### Run

[Install Rust](https://www.rust-lang.org/tools/install) and then:

```shell
cargo run --release -- --help

cargo run --release -- "input dir" 10
```

Debug errors:

```shell
RUST_BACKTRACE=1 cargo run -- FOLDER 8 --force --output "$HOME/Downloads/IMPRO"
```
