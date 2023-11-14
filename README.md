# FDO Impro randomizer

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

```shell
cargo run --release -- --help
```

Debug errors:

```shell
RUST_BACKTRACE=1 cargo run -- FDO 8 --force --output "$HOME/Downloads/FDO"
```
