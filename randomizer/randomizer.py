#!/usr/bin/env python3

"""
FDO Impro randomizer
Akseli Lukkarila
2023
"""

import random
import shutil
import sys
from pathlib import Path

import click

from colorprint import print_bold, print_error, print_warn


def fdo_impro_randomizer(input_path: str, permutations=1, verbose=False):
    """
    Generate randomized play orders for audio files from the given input directory.
    Copies audio files from input folder to new folders with numbered names in the picked random order.
    The permutation parameter controls how many folders to generate.
    """
    input_path = Path(input_path.strip()).resolve()
    if not input_path.exists():
        sys.exit(f"Input directory does not exist: '{input_path}'")

    files: list[Path] = sorted(
        f for f in input_path.iterdir() if f.suffix in (".aif", ".aiff", ".wav", ".flac", ".mp3", ".m4a")
    )
    if not files:
        sys.exit(f"No audio files found in: '{input_path}'")

    files_padding = len(str(len(files)))
    permutations_padding = len(str(permutations))

    print_bold(f"Found {len(files)} audio files in: '{input_path}'")
    if verbose:
        for index, file in enumerate(files, 1):
            print(f"  {index:>{files_padding}}: {file.name}")

    if permutations > 99:
        print_warn(f"That's a lot of permutations ({permutations}), limiting to 99...")
        permutations = 99

    # put output folders to the same parent dir as input
    output_root = input_path.parent
    print_bold(f"Generating {permutations} randomized file permutations to: {output_root}")

    # generate randomized play orders
    for number in range(1, permutations + 1):
        output_name = f"FDO Impro {number:0>{permutations_padding}}"
        output_path = output_root / output_name
        randomized_files = random.sample(files, k=len(files))
        # keep sampling until there are no files from the same artist in consecutive places
        while _check_consecutive_tracks_from_same_artist(randomized_files):
            randomized_files = random.sample(files, k=len(files))

        if output_path.exists():
            print_error("Skipping already existing output path: '{output_path}'")
            continue

        output_path.mkdir()

        # map existing file to new randomized output path
        output_files = [
            (f, output_path / f"{n:0>{files_padding}} FDO impro - {f.name}") for n, f in enumerate(randomized_files, 1)
        ]
        print(f"  Copying to {output_path}...")
        for original_file, new_file in output_files:
            shutil.copy(original_file, new_file)


def _check_consecutive_tracks_from_same_artist(files: list[Path]) -> bool:
    """
    Returns false if there are no consecutive files with the same artist name.
    This assumes all files are named in the format: <artist> - <title>
    """
    if len(files) < 2:
        return False

    for previous, current in zip(files, files[1:]):
        if previous.name.split(" - ", 1)[0].lower() == current.name.split(" - ", 1)[0].lower():
            return True

    return False


@click.command()
@click.help_option("-h", "--help")
@click.argument("input_dir", type=click.Path(exists=True), required=True)
@click.argument("permutations", default=1, type=int)
def main(input_dir: str, permutations: int):
    """
    Generate randomized play orders from a given folder of audio files.

    INPUT_DIR: Input directory.

    PERMUTATIONS: Optional number of permutations to generate (default is 1).
    """
    try:
        fdo_impro_randomizer(input_dir, permutations)
    except KeyboardInterrupt:
        sys.exit("\ncancelled")


if __name__ == "__main__":
    main()
