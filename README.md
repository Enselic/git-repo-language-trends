# git-repo-language-trends
Prints tabulated data about programming language usage over time in a git
repository for a given set of file extensions.

Copy-paste the output into e.g. Google Sheets or Microsoft Excel to easily make
a graph. Stacked area chart is recommended.

# Example
Simply pass the file extensions of the languages you want the trend for to `--filter`.
```
% cd ~/src/your-project
% git-repo-language-trends --filter .cpp .rs
           .cpp    .rs
2021-01-23 121     0
2021-01-22 107     0
2021-01-19 66      0
```

# Installation
## Pre-built binaries
You can download pre-built binaries for **Linux**, **Mac** and **Windows** for the latest release [here](https://github.com/Enselic/git-repo-language-trends/releases).

## cargo install
If you have Rust and Cargo installed, all you need to do to fetch, build and
install the self-contained `git-repo-language-trends` crate is:
```
cargo install git-repo-language-trends
```

## From source
You can of course also clone this repo and then simply `cargo build` it if you have Rust and Cargo installed on your system.

# Performance
This program is very fast. It counts ~5 000 000 lines / second on a high-end
2018 laptop.

This is because the inner loop uses the Rust `libgit2`
[bindings](https://github.com/rust-lang/git2-rs). A regular shell script on a
fast 2018 laptop that uses `git show $COMMIT:$FILE` in the inner loop counts
only ~20 000 lines / second.
