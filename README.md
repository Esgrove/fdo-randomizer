# FDO Impro Randomizer

Generate randomized play orders from a given folder of audio files.

## Dependencies

- Python 3.11+ & Poetry
- Click
- colorama

## Run

```shell
python3 randomizer.py <input_dir> [num_permutations]
```

## Rust version

Run:

```shell
cargo run --release -- --help
```

Usage:

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

Debug errors:

```shell
RUST_BACKTRACE=1 cargo run -- FDO 8 --force --output "$HOME/Downloads/FDO"
```
