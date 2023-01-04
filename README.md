# Koala2MIDI

Convert [Koala](https://www.koalasampler.com) songs to MIDI, e.g. for further processing in a DAW.

Currently in "toy" stage.

## What works:

- Convert a .koala file into a midi file
  - Alternatively, a sequence.json (to be manually extracted from .koala zip file) can be supplied
- All (non-empty) sequences are converted into individual tracks

So, not much ;)

## Usage

```
Usage: koala2midi <INPUT_FILE>

Arguments:
  <INPUT_FILE>  Name of the person to greet

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```