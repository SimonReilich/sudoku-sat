# Instructions

Please unpack the .zip archive. This project uses Nix Flakes for builds.

## Build

To build the default package:
```bash
nix build
```

The binary will be available at `./result/bin/sudoku-sat`.

## Run

To run the application directly:
```bash
nix run
```

To run the tests (using the provided sudokus and sudoku-solutions):
```bash
nix run .#test
```

## Usage

The program expects the specified 81-character input and then returns the result in the same format. This is repeated as long as new input is supplied. To terminate, press Enter after a blank line.