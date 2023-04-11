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

from colorprint import print_warn, print_error, print_bold


def fdo_impro_randomizer(input_path: str, permutations=1, verbose=False):
    """
    Generate randomized play orders for audio files from the given input directory.
    Copies audio files from input folder to new folders with numbered names in the picked random order.
    Permutations parameter controls how many folders to generate.
    """
    input_path = Path(input_path.strip()).resolve()
    if not input_path.exists():
        sys.exit(f"Input directory does not exist: '{input_path}'")

    files: list[Path] = sorted(
        f for f in input_path.iterdir() if f.suffix in (".aif", ".aiff", ".wav", ".flac", ".alac", ".mp3", ".m4a")
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
    for index, file in enumerate(files[1:]):
        artist = file.name.split(" - ")[0]
        previous_artist = files[index].name.split(" - ")[0]
        if artist == previous_artist:
            return True

    return False


if __name__ == "__main__":
    try:
        args = sys.argv[1:]
        input_dir = args[0] or ""
        permutations = int(args[1]) if len(args) > 1 else 1
        fdo_impro_randomizer(input_dir, permutations)
    except KeyboardInterrupt:
        sys.exit("\ninterrupted")
