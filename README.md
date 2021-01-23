# git-repo-language-trends
Prints tabulated data about programming language usage over time in a git
repository for a given set of file extensions.

Copy-paste the output into e.g. Google Sheets or Microsoft Excel to easily make
a graph. Stacked area chart is recommended.

# Example

Simply pass the file extensions of the languages you want the trend for. In the output, `2021w02` means "2021, week 02".
```
% git-repo-language-trends cpp rs
        cpp     rs
2021w03 0       245
2021w02 143     198
2021w01 386     27
```

# Installation

## Pre-built binaries

You can download pre-built binaries for **Linux**, **Mac** and **Windows** for the latest release [here](https://github.com/Enselic/git-repo-language-trends/releases).


## cargo install

If you have Rust and Cargo installed, all you need to do to fetch, build and install the self-contained `git-repo-language-trends` binary from source is:

```
cargo install git-repo-language-trends
```

## From source
You can of course also clone this repo and then simply `cargo build` it if you have Rust and Cargo installed on your system.

# Implementation details

The current implementation spawns lots of `git` child processes for the
processing. Using the [git2-rs library](https://github.com/rust-lang/git2-rs)
directly would probably result in a significant speedup, but at the cost of much
more complicated code.
